extern crate byteorder;


use std::io::Cursor;
use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};

pub fn murmur3_32(source: &mut Read, seed: u32) -> u32 {
    const C1: u32 = 0x85ebca6b;
    const C2: u32 = 0xc2b2ae35;
    const R1: u32 = 16;
    const R2: u32 = 13;
    const M: u32 = 5;
    const N: u32 = 0xe6546b64;
    let mut hash = seed;
    let mut buf = [0; 4];
    let mut processed: u32 = 0;
    loop {
        match source.read(&mut buf[..]) {
            Ok(size) => {
                match size {
                    4 => {
                        let mut tmp = Cursor::new(buf);
                        let k = tmp.read_u32::<LittleEndian>().unwrap();
                        hash ^= calc_k(k);
                        hash = hash.rotate_left(R2);
                        hash = (hash.wrapping_mul(M)).wrapping_add(N);
                    }
                    3 => {
                        let k: u32 = ((buf[2] as u32) << 16) | ((buf[1] as u32) << 8) |
                                     (buf[0] as u32);
                        hash ^= calc_k(k);
                    }
                    2 => {
                        let k: u32 = ((buf[1] as u32) << 8) | (buf[0] as u32);
                        hash ^= calc_k(k);
                    }
                    1 => {
                        let k: u32 = buf[0] as u32;
                        hash ^= calc_k(k);
                    }
                    0 => {
                        hash ^= (processed) as u32;
                        hash ^= hash.wrapping_shr(R1);
                        hash = hash.wrapping_mul(C1);
                        hash ^= hash.wrapping_shr(R2);
                        hash = hash.wrapping_mul(C2);
                        hash ^= hash.wrapping_shr(R1);
                        return hash;
                    }
                    _ => panic!("Invalid read size!"),
                };
                processed += size as u32;
            }
            Err(e) => panic!(e),
        }
    }
}

fn calc_k(k: u32) -> u32 {
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;
    const R1: u32 = 15;
    k.wrapping_mul(C1).rotate_left(R1).wrapping_mul(C2)
}
