/*
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.
*/

use super::big::NLEN;
use crate::arch::Chunk;
use crate::types::{CurvePairingType, CurveType, ModType, SexticTwist, SignOfX};

// Base Bits= 60
// bls461 Modulus

pub const MODULUS: [Chunk; NLEN] = [
    0xAAC0000AAAAAAAB,
    0x20000555554AAAA,
    0x6AA91557F004000,
    0xA8DFFA5C1CC00F2,
    0xACCA47B14848B42,
    0x935FBD6F1E32D8B,
    0xD5A555A55D69414,
    0x15555545554,
];
pub const R2MODP: [Chunk; NLEN] = [
    0x96D08774614DDA8,
    0xCD45F539225D5BD,
    0xD712EB760C95AB1,
    0xB3B687155F30B55,
    0xC4E62A05C3F5B81,
    0xBA1151676CA3CD0,
    0x7EDD8A958F442BE,
    0x12B89DD3F91,
];
pub const MCONST: Chunk = 0xC0005FFFFFFFD;
pub const FRA: [Chunk; NLEN] = [
    0xF7117BF9B812A3A,
    0xA1C6308A599C400,
    0x5A6510E07505BF8,
    0xB31ACE4858D45FA,
    0xFC61EBC2CB04770,
    0x366190D073588E2,
    0x69E55E24DFEFA84,
    0x12E40504B7F,
];
pub const FRB: [Chunk; NLEN] = [
    0xB3AE8410F298071,
    0x7E39D4CAFBAE6A9,
    0x104404777AFE407,
    0xF5C52C13C3EBAF8,
    0xB0685BEE7D443D1,
    0x5CFE2C9EAADA4A8,
    0x6BBFF7807D79990,
    0x27150409D5,
];

// bls461 Curve
pub const CURVE_COF_I: isize = 0;
pub const CURVE_A: isize = 0;
pub const CURVE_B_I: isize = 9;
pub const CURVE_B: [Chunk; NLEN] = [0x9, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x1,
    0x7FEFFFEFFFFC0,
    0xC017FFC80001100,
    0x7FE05FD000E801F,
    0xFFFF7FFFC018001,
    0xFF,
    0x0,
    0x0,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0x14D026A8ADEE93D,
    0xF2D9C00EE74B741,
    0x229C3981B531AC7,
    0x6650D3564DC9218,
    0x436166F7C292A09,
    0x2CF668BE922B197,
    0x463B73A0C813271,
    0xAD0E74E99B,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0xF763157AD1D465,
    0x5D17884C8C4FF47,
    0x9D0A819E66B8D21,
    0x910AE5C3245F495,
    0x96EECB8BFA40B84,
    0x277ACC8BF9F8CBE,
    0x5F68C95F1C3F2F,
    0x77BCDB14B3,
];

pub const CURVE_BNX: [Chunk; NLEN] = [0xFFBFFFE00000000, 0x1FFFF, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_COF: [Chunk; NLEN] = [
    0xAA7FFFEAAAAAAAB,
    0xFFD55AAAB01556A,
    0x1555554FF,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
];
pub const CURVE_CRU: [Chunk; NLEN] = [
    0x40001FFFFFFFE,
    0x6FFFE7FFFFE0000,
    0x6047200C47F0FFF,
    0x777115796DB7BCC,
    0x3F0E89875433CF4,
    0xBFFF60500050261,
    0x1FFFFFE,
    0x0,
];
pub const CURVE_PXA: [Chunk; NLEN] = [
    0x65B503186D0A37C,
    0xA9C2E492E75DCC4,
    0x564E01F919D6878,
    0x3F086DB74FF92F,
    0xED78D46D581A668,
    0x270C892F97C2907,
    0x6A50A9AF679453C,
    0x10CC54138A0,
];
pub const CURVE_PXB: [Chunk; NLEN] = [
    0x9F85CA8C2C1C0AD,
    0x96CD66C425CADE,
    0x1AC612951A2896,
    0xB17D529ABEBEE24,
    0xC5AF5BA09D33F65,
    0x6A672E4D4371ED4,
    0xACEA37CA279D224,
    0x95C1FB4FE5,
];
pub const CURVE_PYA: [Chunk; NLEN] = [
    0x7CCD0C1B02FB006,
    0x953D194A4A12A33,
    0x68B4960CFCC92C8,
    0xBA0F3A9B00F39FC,
    0xCDFD8A7DBBC5ED1,
    0xE73ED227CC2F7A9,
    0xEBA7E676070F4F4,
    0x226AC848E7,
];
pub const CURVE_PYB: [Chunk; NLEN] = [
    0x8A506ADFDF1457C,
    0xB4D6A31DC04C20A,
    0x668EA9A8F136E3F,
    0x12973C3BE4492F5,
    0xA20BE74BEABA67A,
    0x5157F04C42E3856,
    0xBB402EA2AB1D004,
    0xE38101B4FA,
];
pub const CURVE_W: [[Chunk; NLEN]; 2] = [[0; NLEN]; 2];
pub const CURVE_SB: [[[Chunk; NLEN]; 2]; 2] = [[[0; NLEN]; 2]; 2];
pub const CURVE_WB: [[Chunk; NLEN]; 4] = [[0; NLEN]; 4];
pub const CURVE_BB: [[[Chunk; NLEN]; 4]; 4] = [[[0; NLEN]; 4]; 4];

pub const USE_GLV: bool = true;
pub const USE_GS_G2: bool = true;
pub const USE_GS_GT: bool = true;
pub const GT_STRONG: bool = false;

pub const MODBYTES: usize = 58;
pub const BASEBITS: usize = 60;

pub const MODBITS: usize = 461;
pub const MOD8: usize = 3;
pub const MODTYPE: ModType = ModType::NotSpecial;
pub const SH: usize = 19;

pub const CURVETYPE: CurveType = CurveType::Weierstrass;
pub const CURVE_PAIRING_TYPE: CurvePairingType = CurvePairingType::Bls;
pub const SEXTIC_TWIST: SexticTwist = SexticTwist::MType;
pub const ATE_BITS: usize = 78;
pub const SIGN_OF_X: SignOfX = SignOfX::NegativeX;
pub const HASH_TYPE: usize = 32;
pub const AESKEY: usize = 16;