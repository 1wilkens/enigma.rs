
use std::cmp::{min};
use std::vec::Vec;

static MD2_S_Table: &'static [u8] = &[
    0x29, 0x2E, 0x43, 0xC9, 0xA2, 0xD8, 0x7C, 0x01, 0x3D, 0x36, 0x54, 0xA1, 0xEC, 0xF0, 0x06, 0x13,
    0x62, 0xA7, 0x05, 0xF3, 0xC0, 0xC7, 0x73, 0x8C, 0x98, 0x93, 0x2B, 0xD9, 0xBC, 0x4C, 0x82, 0xCA,
    0x1E, 0x9B, 0x57, 0x3C, 0xFD, 0xD4, 0xE0, 0x16, 0x67, 0x42, 0x6F, 0x18, 0x8A, 0x17, 0xE5, 0x12,
    0xBE, 0x4E, 0xC4, 0xD6, 0xDA, 0x9E, 0xDE, 0x49, 0xA0, 0xFB, 0xF5, 0x8E, 0xBB, 0x2F, 0xEE, 0x7A,
    0xA9, 0x68, 0x79, 0x91, 0x15, 0xB2, 0x07, 0x3F, 0x94, 0xC2, 0x10, 0x89, 0x0B, 0x22, 0x5F, 0x21,
    0x80, 0x7F, 0x5D, 0x9A, 0x5A, 0x90, 0x32, 0x27, 0x35, 0x3E, 0xCC, 0xE7, 0xBF, 0xF7, 0x97, 0x03,
    0xFF, 0x19, 0x30, 0xB3, 0x48, 0xA5, 0xB5, 0xD1, 0xD7, 0x5E, 0x92, 0x2A, 0xAC, 0x56, 0xAA, 0xC6,
    0x4F, 0xB8, 0x38, 0xD2, 0x96, 0xA4, 0x7D, 0xB6, 0x76, 0xFC, 0x6B, 0xE2, 0x9C, 0x74, 0x04, 0xF1,
    0x45, 0x9D, 0x70, 0x59, 0x64, 0x71, 0x87, 0x20, 0x86, 0x5B, 0xCF, 0x65, 0xE6, 0x2D, 0xA8, 0x02,
    0x1B, 0x60, 0x25, 0xAD, 0xAE, 0xB0, 0xB9, 0xF6, 0x1C, 0x46, 0x61, 0x69, 0x34, 0x40, 0x7E, 0x0F,
    0x55, 0x47, 0xA3, 0x23, 0xDD, 0x51, 0xAF, 0x3A, 0xC3, 0x5C, 0xF9, 0xCE, 0xBA, 0xC5, 0xEA, 0x26,
    0x2C, 0x53, 0x0D, 0x6E, 0x85, 0x28, 0x84, 0x09, 0xD3, 0xDF, 0xCD, 0xF4, 0x41, 0x81, 0x4D, 0x52,
    0x6A, 0xDC, 0x37, 0xC8, 0x6C, 0xC1, 0xAB, 0xFA, 0x24, 0xE1, 0x7B, 0x08, 0x0C, 0xBD, 0xB1, 0x4A,
    0x78, 0x88, 0x95, 0x8B, 0xE3, 0x63, 0xE8, 0x6D, 0xE9, 0xCB, 0xD5, 0xFE, 0x3B, 0x00, 0x1D, 0x39,
    0xF2, 0xEF, 0xB7, 0x0E, 0x66, 0x58, 0xD0, 0xE4, 0xA6, 0x77, 0x72, 0xF8, 0xEB, 0x75, 0x4B, 0x0A,
    0x31, 0x44, 0x50, 0xB4, 0x8F, 0xED, 0x1F, 0x1A, 0xDB, 0x99, 0x8D, 0x33, 0x9F, 0x11, 0x83, 0x14
];

#[allow(non_camel_case_types)]
pub struct MD2_HashState {
    check_sum : [u8, ..16],
    x         : [u8, ..48],
    buffer    : [u8, ..16],
    cur_len   : uint     // Not sure how big this needs to be!?
}

fn md2_compress(state: &mut MD2_HashState) {

}

fn md2_update_chksum(state: &mut MD2_HashState) {

}

pub fn md2_init() -> Option<MD2_HashState> {
    // Could also impl 'Default' Trait for MD2_HashState
    Some(MD2_HashState {
        check_sum : [0, ..16],
        x         : [0, ..48],
        buffer    : [0, ..16],
        cur_len   : 0
        })
}

pub fn md2_process(state: &mut MD2_HashState, msg_in: Vec<u8>) -> Option<MD2_HashState> {
    // When is this the case?
    if state.cur_len > state.buffer.len() {
       return None
    }

    let mut index = 0u;
    let mut msg_len = msg_in.len();

    loop {
        if msg_len <= 0 {
            break;
        }

        let n = min(msg_len, (16 - state.cur_len));
        for i in range(0, n) {
            state.buffer[state.cur_len + i] = msg_in[index + i];
        }
        state.cur_len += n;
        index         += n;
        msg_len       -= n;

        /* is 16 bytes full? */
        if state.cur_len == 16 {
            md2_compress(state);
            md2_update_chksum(state);
            state.cur_len = 0;
        }
    }

    Some(*state)
}

pub fn md2_done(state: &mut MD2_HashState) -> Option<[u8, ..16]> {

    if state.cur_len >= state.buffer.len() {
       return None;
    }

    /* pad the message */
    let k: u8 = 16u8 - state.cur_len as u8;
    for i in range(state.cur_len, 16) {
        state.buffer[i] = k;
    }

    /* hash and update */
    md2_compress(state);
    md2_update_chksum(state);

    /* hash checksum */
    for i in range(0, 16) {
        state.buffer[i] = state.check_sum[i];
    }
    md2_compress(state);

    let mut result = [0u8, ..16];
    /* output is lower 16 bytes of X */
    for i in range(0, 16) {
        result[i] = state.x[i];
    }
    Some(result)
}

#[test]
fn test_md2() {
    let tests: Vec<(String, [u8, ..16])> = vec![
        ("".to_string(), [
            0x83,0x50,0xe5,0xa3,0xe2,0x4c,0x15,0x3d,
            0xf2,0x27,0x5c,0x9f,0x80,0x69,0x27,0x73
        ]),
        ("a".to_string(), [
            0x32,0xec,0x01,0xec,0x4a,0x6d,0xac,0x72,
            0xc0,0xab,0x96,0xfb,0x34,0xc0,0xb5,0xd1
        ]),
        ("message digest".to_string(), [
            0xab,0x4f,0x49,0x6b,0xfb,0x2a,0x53,0x0b,
            0x21,0x9f,0xf3,0x30,0x31,0xfe,0x06,0xb0
        ]),
        ("abcdefghijklmnopqrstuvwxyz".to_string(), [
            0x4e,0x8d,0xdf,0xf3,0x65,0x02,0x92,0xab,
            0x5a,0x41,0x08,0xc3,0xaa,0x47,0x94,0x0b
        ]),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string(), [
            0xda,0x33,0xde,0xf2,0xa4,0x2d,0xf1,0x39,
            0x75,0x35,0x28,0x46,0xc3,0x03,0x38,0xcd
        ]),
        ("12345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string(), [
            0xd5,0x97,0x6f,0x79,0xd8,0x3d,0x3a,0x0d,
            0xc9,0x80,0x6c,0x3c,0x66,0xf3,0xef,0xd8
        ])
    ];

    for &test in tests.iter() {
        let (msg, digest) = test;
        let mut state = md2_init().unwrap();
        state = md2_process(&mut state, msg.into_bytes()).unwrap();
        let calculated = md2_done(&mut state).unwrap();
        for i in range(0, 15) {
            assert!(digest[i] == calculated[i])
        }
    }
}
