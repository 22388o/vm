#![allow(dead_code)]

type Bytes = Vec<u8>;

// https://en.bitcoin.it/wiki/Protocol_documentation#Variable_length_integer
pub fn with_prefix_compact_size(data: &Bytes) -> Bytes {
    let mut return_vec: Bytes = Vec::<u8>::new();

    let data_len = data.len();

    match data_len {
        0..=252 => return_vec.extend(vec![data_len as u8]),
        253..=65535 => {
            return_vec.extend(vec![0xfd]);
            let vec_u8: Bytes = vec![(data_len & 0xFF) as u8, (data_len >> 8 & 0xFF) as u8];
            return_vec.extend(vec_u8)
        }
        65536..=4294967295 => {
            return_vec.extend(vec![0xfe]);
            let vec_u8: Bytes = vec![
                (data_len & 0xFF) as u8,
                ((data_len >> 8) & 0xFF) as u8,
                ((data_len >> 16) & 0xFF) as u8,
                ((data_len >> 24) & 0xFF) as u8,
            ];
            return_vec.extend(vec_u8)
        }
        4294967296..=0xFFFFFFFFFFFFFFFF => {
            return_vec.extend(vec![0xff]);
            let vec_u8: Bytes = vec![
                (data_len & 0xFF) as u8,
                ((data_len >> 8) & 0xFF) as u8,
                ((data_len >> 16) & 0xFF) as u8,
                ((data_len >> 24) & 0xFF) as u8,
                ((data_len >> 32) & 0xFF) as u8,
                ((data_len >> 40) & 0xFF) as u8,
                ((data_len >> 48) & 0xFF) as u8,
                ((data_len >> 56) & 0xFF) as u8,
            ];
            return_vec.extend(vec_u8)
        }
        _ => panic!(),
    }
    return_vec.extend(data);
    return_vec
}

// https://en.bitcoin.it/wiki/Script
pub fn with_prefix_pushdata(data: &Bytes) -> Bytes {
    let mut return_vec: Bytes = Vec::<u8>::new();

    if data.len() == 1 && &data[0] <= &16 {
        // Minimal push
        match &data[0] {
            0x00 => return_vec.push(0x00), // OP_0
            0x01 => return_vec.push(0x51), // OP_1
            0x02 => return_vec.push(0x52), // OP_2
            0x03 => return_vec.push(0x53), // OP_3
            0x04 => return_vec.push(0x54), // OP_4
            0x05 => return_vec.push(0x55), // OP_5
            0x06 => return_vec.push(0x56), // OP_6
            0x07 => return_vec.push(0x57), // OP_7
            0x08 => return_vec.push(0x58), // OP_8
            0x09 => return_vec.push(0x59), // OP_9
            0x0a => return_vec.push(0x5a), // OP_10
            0x0b => return_vec.push(0x5b), // OP_11
            0x0c => return_vec.push(0x5c), // OP_12
            0x0d => return_vec.push(0x5d), // OP_13
            0x0e => return_vec.push(0x5e), // OP_14
            0x0f => return_vec.push(0x5f), // OP_15
            0x10 => return_vec.push(0x60), // OP_16
            _ => panic!(),
        }
        return_vec
    } else {
        match data.len() {
            x if x <= 75 => return_vec.extend(vec![x as u8]),
            x if x <= 0xFF => {
                return_vec.extend(vec![0x4c]);
                return_vec.extend(vec![x as u8])
            }
            x if x <= 0xFFFF => {
                return_vec.extend(vec![0x4d]);

                let vec_u8: Bytes = vec![(x & 0xFF) as u8, (x >> 8 & 0xFF) as u8];

                return_vec.extend(vec_u8)
            }
            x if x <= 0xFFFFFFFF => {
                return_vec.extend(vec![0x4e]);

                // In little endian order
                let vec_u8: Bytes = vec![
                    (x & 0xFF) as u8,
                    ((x >> 8) & 0xFF) as u8,
                    ((x >> 16) & 0xFF) as u8,
                    ((x >> 24) & 0xFF) as u8,
                ];

                return_vec.extend(vec_u8)
            }
            _ => panic!(),
        }
        return_vec.extend(data);
        return_vec
    }
}