// Copyright (c) 2020 Stu Small
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::cmp::min;
use std::io::{Read, Result};

use crate::read_bytes;

const C1: u32 = 0x85eb_ca6b;
const C2: u32 = 0xc2b2_ae35;
const R1: u32 = 16;
const R2: u32 = 13;
const M: u32 = 5;
const N: u32 = 0xe654_6b64;

/// Use the 32 bit variant of murmur3 to hash some [Read] implementation.
///
/// # Example
/// ```
/// use std::io::Cursor;
/// use murmur3::murmur3_32;
/// let hash_result = murmur3_32(&mut Cursor::new("hello world"), 0);
/// ```
pub fn murmur3_32<T: Read>(source: &mut T, seed: u32) -> Result<u32> {
    let mut buffer: [u8; 4] = [0; 4];
    let mut processed = 0;
    let mut state = seed;
    loop {
        match read_bytes(source, &mut buffer)? {
            4 => {
                processed += 4;
                let k = u32::from_le_bytes(buffer);
                state ^= calc_k(k);
                state = state.rotate_left(R2);
                state = (state.wrapping_mul(M)).wrapping_add(N);
            }
            3 => {
                processed += 3;
                let k: u32 =
                    ((buffer[2] as u32) << 16) | ((buffer[1] as u32) << 8) | (buffer[0] as u32);
                state ^= calc_k(k);
            }
            2 => {
                processed += 2;
                let k: u32 = ((buffer[1] as u32) << 8) | (buffer[0] as u32);
                state ^= calc_k(k);
            }
            1 => {
                processed += 1;
                let k: u32 = buffer[0] as u32;
                state ^= calc_k(k);
            }
            0 => return Ok(finish(state, processed)),
            _ => panic!("Internal buffer state failure"),
        }
    }
}

/// Use the 32 bit variant of murmur3 to hash [u8] without copying the buffer.
///
/// # Example
///
/// ```
/// use murmur3::murmur3_32_of_slice;
/// let hash_result = murmur3_32_of_slice("hello world".as_bytes(), 0);
/// ```
pub fn murmur3_32_of_slice(source: &[u8], seed: u32) -> u32 {
    let mut buffer = source;
    let mut processed = 0;
    let mut state = seed;
    loop {
        match min(buffer.len(), 4) {
            0 => return finish(state, processed),
            1 => {
                processed += 1;
                let k: u32 = buffer[0] as u32;
                state ^= calc_k(k);
                return finish(state, processed);
            }
            2 => {
                processed += 2;
                let k: u32 = ((buffer[1] as u32) << 8) | (buffer[0] as u32);
                state ^= calc_k(k);
                return finish(state, processed);
            }
            3 => {
                processed += 3;
                let k: u32 =
                    ((buffer[2] as u32) << 16) | ((buffer[1] as u32) << 8) | (buffer[0] as u32);
                state ^= calc_k(k);
                return finish(state, processed);
            }
            4 => {
                processed += 4;
                let k: u32 = ((buffer[3] as u32) << 24)
                    | ((buffer[2] as u32) << 16)
                    | ((buffer[1] as u32) << 8)
                    | (buffer[0] as u32);
                state ^= calc_k(k);
                state = state.rotate_left(R2);
                state = (state.wrapping_mul(M)).wrapping_add(N);
                buffer = &buffer[4..];
            }
            _ => unreachable!(),
        };
    }
}

/// Use the 32 bit variant of murmur3 to hash [[u8],..] without copying the buffers.
///
/// # Example
///
/// ```
/// use murmur3::murmur3_32_of_slices;
/// let hash_result = murmur3_32_of_slices(&["hello".as_bytes()," world".as_bytes()], 0);
/// ```
pub fn murmur3_32_of_slices(source: &[&[u8]], seed: u32) -> u32 {
    let mut state = seed;
    let mut processed:usize = 0;
    let mut k: u32=0u32;
    let mut k_offset: u8=0u8;
    for slice in source {
        let mut buffer = *slice;
        let mut buffer_len = buffer.len();
        processed += buffer_len;
        while buffer_len != 0 {
            match k_offset {
                0 => {
                    match buffer_len {
                        3 => {
                            k |= (buffer[2] as u32) << 16 | (buffer[1] as u32) << 8 | (buffer[0] as u32);
                            buffer = &buffer[3..];
                            k_offset = 3;
                        },
                        2 => {
                            k |= (buffer[1] as u32) << 8 | (buffer[0] as u32);
                            buffer = &buffer[2..];
                            k_offset = 2;
                        },
                        1 => {
                            k |= buffer[0] as u32;
                            buffer = &buffer[1..];
                            k_offset = 1;
                        },
                        _ => {
                            k = ((buffer[3] as u32) << 24)
                                | ((buffer[2] as u32) << 16)
                                | ((buffer[1] as u32) << 8)
                                | (buffer[0] as u32);
                            buffer = &buffer[4..];
                            k_offset = 0;
                        }
                    }
                },
                1 => {
                    match buffer_len {
                        2 => {
                            k |= (buffer[1] as u32) << 16 | (buffer[0] as u32) <<8;
                            buffer = &buffer[2..];
                            k_offset = 3;
                        },
                        1 => {
                            k |= (buffer[0] as u32)<<8;
                            buffer = &buffer[1..];
                            k_offset = 2;
                        },
                        _ => {
                            k |= (buffer[2] as u32) << 24 | (buffer[1] as u32) << 16 | (buffer[0] as u32) <<8;
                            buffer = &buffer[3..];
                            k_offset = 0;
                        },
                    }
                },
                2 => {
                    match buffer_len {
                        1 => {
                            k |= (buffer[0] as u32)<<16;
                            buffer = &buffer[1..];
                            k_offset = 3;
                        },
                        _ => {
                            k |= (buffer[1] as u32) << 24 | (buffer[0] as u32)<<16;
                            buffer = &buffer[2..];
                            k_offset = 0;
                        },
                    }
                },
                3 => {
                    k |= (buffer[0] as u32)<<24;
                    buffer = &buffer[1..];
                    k_offset = 0;
                    }
                _ => unreachable!()
            }

            if k_offset == 0 {
                // all 4-bytes ware collected
                state ^= calc_k(k);
                state = state.rotate_left(R2);
                state = (state.wrapping_mul(M)).wrapping_add(N);
                k = 0u32;
            }
            buffer_len = buffer.len();
        }
    }

    if k_offset != 0 {
        // k still has some unprocessed data
        state ^= calc_k(k);
    }

    finish(state, processed as u32)
}

fn finish(state: u32, processed: u32) -> u32 {
    let mut hash = state;
    hash ^= processed;
    hash ^= hash.wrapping_shr(R1);
    hash = hash.wrapping_mul(C1);
    hash ^= hash.wrapping_shr(R2);
    hash = hash.wrapping_mul(C2);
    hash ^= hash.wrapping_shr(R1);
    hash
}

fn calc_k(k: u32) -> u32 {
    const C1: u32 = 0xcc9e_2d51;
    const C2: u32 = 0x1b87_3593;
    const R1: u32 = 15;
    k.wrapping_mul(C1).rotate_left(R1).wrapping_mul(C2)
}
