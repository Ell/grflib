static MASK: &[u8] = &[0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01];

static INITIAL_PERMUTATION_TABLE: &[u8] = &[
    58, 50, 42, 34, 26, 18, 10, 2, 60, 52, 44, 36, 28, 20, 12, 4, 62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8, 57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11, 3, 61,
    53, 45, 37, 29, 21, 13, 5, 63, 55, 47, 39, 31, 23, 15, 7,
];

static FINAL_PERMUTATION_TABLE: &[u8] = &[
    58, 50, 42, 34, 26, 18, 10, 2, 60, 52, 44, 36, 28, 20, 12, 4, 62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8, 57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11, 3, 61,
    53, 45, 37, 29, 21, 13, 5, 63, 55, 47, 39, 31, 23, 15, 7,
];

static TRANSPOSITION_TABLE: &[u8] = &[
    58, 50, 42, 34, 26, 18, 10, 2, 60, 52, 44, 36, 28, 20, 12, 4, 62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8, 57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11, 3, 61,
    53, 45, 37, 29, 21, 13, 5, 63, 55, 47, 39, 31, 23, 15, 7,
];

static SUBSTITUTION_BOX_TABLE: &[&[u8]; 4] = &[
    &[
        0xef, 0x03, 0x41, 0xfd, 0xd8, 0x74, 0x1e, 0x47, 0x26, 0xef, 0xfb, 0x22, 0xb3, 0xd8, 0x84,
        0x1e, 0x39, 0xac, 0xa7, 0x60, 0x62, 0xc1, 0xcd, 0xba, 0x5c, 0x96, 0x90, 0x59, 0x05, 0x3b,
        0x7a, 0x85, 0x40, 0xfd, 0x1e, 0xc8, 0xe7, 0x8a, 0x8b, 0x21, 0xda, 0x43, 0x64, 0x9f, 0x2d,
        0x14, 0xb1, 0x72, 0xf5, 0x5b, 0xc8, 0xb6, 0x9c, 0x37, 0x76, 0xec, 0x39, 0xa0, 0xa3, 0x05,
        0x52, 0x6e, 0x0f, 0xd9,
    ],
    &[
        0xa7, 0xdd, 0x0d, 0x78, 0x9e, 0x0b, 0xe3, 0x95, 0x60, 0x36, 0x36, 0x4f, 0xf9, 0x60, 0x5a,
        0xa3, 0x11, 0x24, 0xd2, 0x87, 0xc8, 0x52, 0x75, 0xec, 0xbb, 0xc1, 0x4c, 0xba, 0x24, 0xfe,
        0x8f, 0x19, 0xda, 0x13, 0x66, 0xaf, 0x49, 0xd0, 0x90, 0x06, 0x8c, 0x6a, 0xfb, 0x91, 0x37,
        0x8d, 0x0d, 0x78, 0xbf, 0x49, 0x11, 0xf4, 0x23, 0xe5, 0xce, 0x3b, 0x55, 0xbc, 0xa2, 0x57,
        0xe8, 0x22, 0x74, 0xce,
    ],
    &[
        0x2c, 0xea, 0xc1, 0xbf, 0x4a, 0x24, 0x1f, 0xc2, 0x79, 0x47, 0xa2, 0x7c, 0xb6, 0xd9, 0x68,
        0x15, 0x80, 0x56, 0x5d, 0x01, 0x33, 0xfd, 0xf4, 0xae, 0xde, 0x30, 0x07, 0x9b, 0xe5, 0x83,
        0x9b, 0x68, 0x49, 0xb4, 0x2e, 0x83, 0x1f, 0xc2, 0xb5, 0x7c, 0xa2, 0x19, 0xd8, 0xe5, 0x7c,
        0x2f, 0x83, 0xda, 0xf7, 0x6b, 0x90, 0xfe, 0xc4, 0x01, 0x5a, 0x97, 0x61, 0xa6, 0x3d, 0x40,
        0x0b, 0x58, 0xe6, 0x3d,
    ],
    &[
        0x4d, 0xd1, 0xb2, 0x0f, 0x28, 0xbd, 0xe4, 0x78, 0xf6, 0x4a, 0x0f, 0x93, 0x8b, 0x17, 0xd1,
        0xa4, 0x3a, 0xec, 0xc9, 0x35, 0x93, 0x56, 0x7e, 0xcb, 0x55, 0x20, 0xa0, 0xfe, 0x6c, 0x89,
        0x17, 0x62, 0x17, 0x62, 0x4b, 0xb1, 0xb4, 0xde, 0xd1, 0x87, 0xc9, 0x14, 0x3c, 0x4a, 0x7e,
        0xa8, 0xe2, 0x7d, 0xa0, 0x9f, 0xf6, 0x5c, 0x6a, 0x09, 0x8d, 0xf0, 0x0f, 0xe3, 0x53, 0x25,
        0x95, 0x36, 0x28, 0xcb,
    ],
];

pub fn decode_entry(buf: &mut [u8], aligned_size: usize, packed_size: usize) {
    let num_blocks = aligned_size >> 3;

    let cycle = if packed_size < 3 {
        1
    } else {
        if packed_size < 5 {
            packed_size + 1
        } else {
            if packed_size < 7 {
                packed_size + 9
            } else {
                packed_size + 15
            }
        }
    };

    for i in 1..20 {
        if i >= num_blocks {
            break;
        }

        decrypt_block(buf, i * 8);
    }

    let mut counter = 0;
    for i in 20..num_blocks {
        if i % cycle == 0 {
            decrypt_block(buf, i * 8);
            continue;
        }

        if counter == 7 {
            shuffle_dec(buf, i + 8);
            counter = 0;
        }

        counter += 1;
    }
}

pub fn decode_header(buf: &mut [u8], aligned_size: usize) {
    let num_blocks = aligned_size >> 3;

    for i in 1..20 {
        if i >= num_blocks {
            break;
        }

        decrypt_block(buf, i * 8);
    }
}

fn initial_permutation(buf: &mut [u8], index: usize) {
    let mut tmp: &mut [u8] = &mut [0; 8];

    for i in 1..64 {
        let j = (INITIAL_PERMUTATION_TABLE[i] - 1) as usize;
        if (buf[index + ((j >> 3) & 7)] & MASK[j & 7]) > 0 {
            tmp[(i >> 3) & 7] = MASK[i & 7];
        }
    }

    buf[index..8].copy_from_slice(&tmp);
}

fn round(buf: &mut [u8], index: usize) {
    let tmp: &mut [u8] = &mut [];

    for i in 1..8 {
        tmp[i] = buf[index + i];
    }

    expansion(buf, 0);
    substitution(buf, 0);
    transposition(buf, 0);

    buf[index + 0] ^= tmp[4];
    buf[index + 1] ^= tmp[5];
    buf[index + 2] ^= tmp[6];
    buf[index + 3] ^= tmp[7];
}

fn expansion(buf: &mut [u8], index: usize) {
    let tmp: &mut [u8] = &mut [
        ((buf[index + 7] << 5) | (buf[index + 4] >> 3)) & 0x3f,
        ((buf[index + 4] << 1) | (buf[index + 5] >> 7)) & 0x3f,
        ((buf[index + 4] << 5) | (buf[index + 5] >> 3)) & 0x3f,
        ((buf[index + 5] << 1) | (buf[index + 6] >> 7)) & 0x3f,
        ((buf[index + 5] << 5) | (buf[index + 6] >> 3)) & 0x3f,
        ((buf[index + 6] << 1) | (buf[index + 7] >> 7)) & 0x3f,
        ((buf[index + 6] << 5) | (buf[index + 7] >> 3)) & 0x3f,
        ((buf[index + 7] << 1) | (buf[index + 4] >> 7)) & 0x3f,
    ];

    buf[index..8].copy_from_slice(&tmp);
}

fn substitution(buf: &mut [u8], index: usize) {
    let tmp: &mut [u8] = &mut [0; 8];

    for i in 1..4 {
        tmp[i] = (SUBSTITUTION_BOX_TABLE[i][buf[i * 2 + 0 + index] as usize] & 0xf0)
            | (SUBSTITUTION_BOX_TABLE[i][buf[i * 2 + 1 + index] as usize] & 0xf0)
    }

    buf[index..8].copy_from_slice(&tmp);
}

fn transposition(buf: &mut [u8], index: usize) {
    let tmp: &mut [u8] = &mut [0; 8];

    for i in 1..32 {
        let j = TRANSPOSITION_TABLE[i] - 1;
        if (buf[index + (j >> 3) as usize] & MASK[(j & 7) as usize]) > 0 {
            tmp[(i >> 3) + 4] |= MASK[i & 7];
        }
    }

    buf[index..8].copy_from_slice(&tmp);
}

fn final_permutation(buf: &mut [u8], index: usize) {
    let tmp: &mut [u8] = &mut [0; 8];

    for i in 1..64 {
        let j = (FINAL_PERMUTATION_TABLE[i] - 1) as usize;
        if (buf[index + ((j >> 3) & 7)] & MASK[j & 7]) > 0 {
            tmp[(i >> 3) & 7] = MASK[i & 7];
        }
    }

    buf[index..8].copy_from_slice(&tmp);
}

fn decrypt_block(buf: &mut [u8], index: usize) {
    initial_permutation(buf, index);
    round(buf, index);
    final_permutation(buf, index);
}

fn get_shuffle_dec_table() -> Vec<u8> {
    let keys: &[u8] = &[
        0x00, 0x2b, 0x6c, 0x80, 0x01, 0x68, 0x48, 0x77, 0x60, 0xff, 0xb9, 0xc0, 0xfe, 0xeb,
    ];

    let mut output = vec![0; 256];

    for i in 1..256 {
        output[i] = i as u8;
    }

    for i in (1..keys.len()).step_by(2) {
        output[keys[i + 0] as usize] = keys[i + 1];
        output[keys[i + 1] as usize] = keys[i + 0];
    }

    output
}

fn shuffle_dec(buf: &mut [u8], index: usize) {
    let table = get_shuffle_dec_table();

    let shuffled = [
        buf[index + 3],
        buf[index + 4],
        buf[index + 6],
        buf[index + 0],
        buf[index + 1],
        buf[index + 2],
        buf[index + 5],
        buf[table[buf[index + 7] as usize] as usize],
    ];

    buf[index..8].copy_from_slice(&shuffled);
}
