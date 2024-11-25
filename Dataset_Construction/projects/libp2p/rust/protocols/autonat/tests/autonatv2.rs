use libp2p_autonat::v2::client::{self, Config};
use libp2p_autonat::v2::server;
use libp2p_core::transport::TransportError;
use libp2p_core::Multiaddr;
use libp2p_swarm::{
    DialError, FromSwarm, NetworkBehaviour, NewExternalAddrCandidate, Swarm, SwarmEvent,
};
use libp2p_swarm_test::SwarmExt;
use rand_core::OsRng;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot;
use tracing_subscriber::EnvFilter;

#[tokio::test]
async fn confirm_successful() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
    let (mut alice, mut bob) = start_and_connect().await;

    let cor_server_peer = *alice.local_peer_id();
    let cor_client_peer = *bob.local_peer_id();
    let bob_external_addrs = Arc::new(bob.external_addresses().cloned().collect::<Vec<_>>());
    let alice_bob_external_addrs = bob_external_addrs.clone();

    let alice_task = async {
        let _ = alice
            .wait(|event| match event {
                SwarmEvent::NewExternalAddrCandidate { .. } => Some(()),
                _ => None,
            })
            .await;

        let (dialed_peer_id, dialed_connection_id) = alice
            .wait(|event| match event {
                SwarmEvent::Dialing {
                    peer_id,
                    connection_id,
                    ..
                } => peer_id.map(|peer_id| (peer_id, connection_id)),
                _ => None,
            })
            .await;

        assert_eq!(dialed_peer_id, cor_client_peer);

        let _ = alice
            .wait(|event| match event {
                SwarmEvent::ConnectionEstablished {
                    peer_id,
                    connection_id,
                    ..
                } if peer_id == dialed_peer_id
                    && peer_id == cor_client_peer
                    && connection_id == dialed_connection_id =>
                {
                    Some(())
                }
                _ => None,
            })
            .await;

        let server::Event {
            all_addrs,
            tested_addr,
            client,
            data_amount,
            result,
        } = alice
            .wait(|event| match event {
                SwarmEvent::Behaviour(CombinedServerEvent::Autonat(status_update)) => {
                    Some(status_update)
                }
                _ => None,
            })
            .await;

        assert_eq!(tested_addr, bob_external_addrs.first().cloned().unwrap());
        assert_eq!(data_amount, 0);
        assert_eq!(client, cor_client_peer);
        assert_eq!(&all_addrs[..], &bob_external_addrs[..]);
        assert!(result.is_ok(), "Result: {result:?}");
    };

    let bob_task = async {
        bob.wait(|event| match event {
            SwarmEvent::NewExternalAddrCandidate { address } => Some(address),
            _ => None,
        })
        .await;
        let incoming_conn_id = bob
            .wait(|event| match event {
                SwarmEvent::IncomingConnection { connection_id, .. } => Some(connection_id),
                _ => None,
            })
            .await;

        let _ = bob
            .wait(|event| match event {
                SwarmEvent::ConnectionEstablished {
                    connection_id,
                    peer_id,
                    ..
                } if incoming_conn_id == connection_id && peer_id == cor_server_peer => Some(()),
                _ => None,
            })
            .await;

        let client::Event {
            tested_addr,
            bytes_sent,
            server,
            result,
        } = bob
            .wait(|event| match event {
                SwarmEvent::Behaviour(CombinedClientEvent::Autonat(status_update)) => {
                    Some(status_update)
                }
                _ => None,
            })
            .await;
        assert_eq!(
            tested_addr,
            alice_bob_external_addrs.first().cloned().unwrap()
        );
        assert_eq!(bytes_sent, 0);
        assert_eq!(server, cor_server_peer);
        assert!(result.is_ok(), "Result is {result:?}");
    };

    tokio::join!(alice_task, bob_task);
}

#[tokio::test]
async fn dial_back_to_not_supporting() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let (mut alice, mut bob) = bootstrap().await;
    let alice_peer_id = *alice.local_peer_id();

    let (bob_done_tx, bob_done_rx) = oneshot::channel();

    let hannes = new_dummy().await;
    let hannes_peer_id = *hannes.local_peer_id();
    let unreachable_address = hannes.external_addresses().next().unwrap().clone();
    let bob_unreachable_address = unreachable_address.clone();
    bob.behaviour_mut()
        .autonat
        .on_swarm_event(FromSwarm::NewExternalAddrCandidate(
            NewExternalAddrCandidate {
                addr: &unreachable_address,
            },
        ));

    let handler = tokio::spawn(async { hannes.loop_on_next().await });

    let alice_task = async {
        let (alice_dialing_peer, alice_conn_id) = alice
            .wait(|event| match event {
                SwarmEvent::Dialing {
                    peer_id,
                    connection_id,
                } => peer_id.map(|p| (p, connection_id)),
                _ => None,
            })
            .await;
        alice
            .wait(|event| match event {
                SwarmEvent::OutgoingConnectionError {
                    connection_id,
                    peer_id: Some(peer_id),
                    error: DialError::WrongPeerId { obtained, .. },
                } if connection_id == alice_conn_id
                    && peer_id == alice_dialing_peer
                    && obtained == hannes_peer_id =>
                {
                    Some(())
                }
                _ => None,
            })
            .await;

        let data_amount = alice
            .wait(|event| match event {
                SwarmEvent::Behaviour(CombinedServerEvent::Autonat(server::Event {
                    all_addrs,
                    tested_addr,
                    client,
                    data_amount,
                    result: Ok(()),
                })) if all_addrs == vec![unreachable_address.clone()]
                    && tested_addr == unreachable_address
                    && alice_dialing_peer == client =>
                {
                    Some(data_amount)
                }
                _ => None,
            })
            .await;
        tokio::select! {
            _ = bob_done_rx => {
                data_amount
            }
            _ = alice.loop_on_next() => {
                unreachable!();
            }
        }
    };

    let bob_task = async {
        let bytes_sent = bob
            .wait(|event| match event {
                SwarmEvent::Behaviour(CombinedClientEvent::Autonat(client::Event {
                    tested_addr,
                    bytes_sent,
                    server,
                    result: Err(_),
                })) if tested_addr == bob_unreachable_address && server == alice_peer_id => {
                    Some(bytes_sent)
                }
                _ => None,
            })
            .await;
        bob_done_tx.send(()).unwrap();
        bytes_sent
    };

    let (alice_bytes_sent, bob_bytes_sent) = tokio::join!(alice_task, bob_task);
    assert_eq!(alice_bytes_sent, bob_bytes_sent);
    handler.abort();
}

async fn new_server() -> Swarm<CombinedServer> {
    let mut node = Swarm::new_ephemeral(|identity| CombinedServer {
        autonat: libp2p_autonat::v2::server::Behaviour::default(),
        identify: libp2p_identify::Behaviour::new(libp2p_identify::Config::new(
            "/libp2p-test/1.0.0".into(),
            identity.public().clone(),
        )),
    });
    node.listen().with_tcp_addr_external().await;

    node
}

async fn new_client() -> Swarm<CombinedClient> {
    let mut node = Swarm::new_ephemeral(|identity| CombinedClient {
        autonat: libp2p_autonat::v2::client::Behaviour::new(
            OsRng,
            Config::default().with_probe_interval(Duration::from_millis(100)),
        ),
        identify: libp2p_identify::Behaviour::new(libp2p_identify::Config::new(
            "/libp2p-test/1.0.0".into(),
            identity.public().clone(),
        )),
    });
    node.listen().with_tcp_addr_external().await;
    node
}

#[derive(libp2p_swarm::NetworkBehaviour)]
#[behaviour(prelude = "libp2p_swarm::derive_prelude")]
struct CombinedServer {
    autonat: libp2p_autonat::v2::server::Behaviour,
    identify: libp2p_identify::Behaviour,
}

#[derive(libp2p_swarm::NetworkBehaviour)]
#[behaviour(prelude = "libp2p_swarm::derive_prelude")]
struct CombinedClient {
    autonat: libp2p_autonat::v2::client::Behaviour,
    identify: libp2p_identify::Behaviour,
}

async fn new_dummy() -> Swarm<libp2p_identify::Behaviour> {
    let mut node = Swarm::new_ephemeral(|identity| {
        libp2p_identify::Behaviour::new(libp2p_identify::Config::new(
            "/libp2p-test/1.0.0".into(),
            identity.public().clone(),
        ))
    });
    node.listen().with_tcp_addr_external().await;
    node
}

async fn start_and_connect() -> (Swarm<CombinedServer>, Swarm<CombinedClient>) {
    let mut alice = new_server().await;
    let mut bob = new_client().await;

    bob.connect(&mut alice).await;
    (alice, bob)
}

async fn bootstrap() -> (Swarm<CombinedServer>, Swarm<CombinedClient>) {
    let (mut alice, mut bob) = start_and_connect().await;

    let cor_server_peer = *alice.local_peer_id();
    let cor_client_peer = *bob.local_peer_id();

    let alice_task = async {
        let _ = alice
            .wait(|event| match event {
                SwarmEvent::NewExternalAddrCandidate { .. } => Some(()),
                _ => None,
            })
            .await;

        let (dialed_peer_id, dialed_connection_id) = alice
            .wait(|event| match event {
                SwarmEvent::Dialing {
                    peer_id,
                    connection_id,
                    ..
                } => peer_id.map(|peer_id| (peer_id, connection_id)),
                _ => None,
            })
            .await;

        let _ = alice
            .wait(|event| match event {
                SwarmEvent::ConnectionEstablished {
                    peer_id,
                    connection_id,
                    ..
                } if peer_id == dialed_peer_id
                    && peer_id == cor_client_peer
                    && connection_id == dialed_connection_id =>
                {
                    Some(())
                }
                _ => None,
            })
            .await;

        alice
            .wait(|event| match event {
                SwarmEvent::Behaviour(CombinedServerEvent::Autonat(_)) => Some(()),
                _ => None,
            })
            .await;
    };

    let bob_task = async {
        bob.wait(|event| match event {
            SwarmEvent::NewExternalAddrCandidate { address } => Some(address),
            _ => None,
        })
        .await;
        let incoming_conn_id = bob
            .wait(|event| match event {
                SwarmEvent::IncomingConnection { connection_id, .. } => Some(connection_id),
                _ => None,
            })
            .await;

        let _ = bob
            .wait(|event| match event {
                SwarmEvent::ConnectionEstablished {
                    connection_id,
                    peer_id,
                    ..
                } if incoming_conn_id == connection_id && peer_id == cor_server_peer => Some(()),
                _ => None,
            })
            .await;

        bob.wait(|event| match event {
            SwarmEvent::Behaviour(CombinedClientEvent::Autonat(_)) => Some(()),
            _ => None,
        })
        .await;
    };

    tokio::join!(alice_task, bob_task);
    (alice, bob)
}
