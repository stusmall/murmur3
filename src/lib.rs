extern crate byteorder;


use std::io::Cursor;
use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};


pub fn murmur3_32(source: &mut Read, seed: u32) -> u32 {
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;
    const R1: u32 = 15;
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
                        let mut k = tmp.read_u32::<LittleEndian>().unwrap();
                        k = k.wrapping_mul(C1);
                        k = k.rotate_left(R1);
                        k = k.wrapping_mul(C2);
                        hash ^= k;
                        hash = hash.rotate_left(R2);
                        hash = (hash.wrapping_mul(M)).wrapping_add(N);
                        processed += 4;
                    }
                    3 => {
                        let mut k: u32 = 0;
                        k ^= (buf[2] as u32) << 16;
                        k ^= (buf[1] as u32) << 8;
                        k ^= buf[0] as u32;
                        k = k.wrapping_mul(C1);
                        k = k.rotate_left(15);
                        k = k.wrapping_mul(C2);
                        processed += 3;
                        hash ^= k;
                    }
                    2 => {
                        let mut k: u32 = 0;
                        k ^= (buf[1] as u32) << 8;
                        k ^= buf[0] as u32;
                        k = k.wrapping_mul(C1);
                        k = k.rotate_left(15);
                        k = k.wrapping_mul(C2);
                        processed += 2;
                        hash ^= k;
                    }
                    1 => {
                        let mut k: u32 = 0;
                        k ^= buf[0] as u32;
                        k = k.wrapping_mul(C1);
                        k = k.rotate_left(15);
                        k = k.wrapping_mul(C2);
                        processed += 1;
                        hash ^= k;
                    }
                    0 => {
                        hash ^= (processed) as u32;
                        hash ^= hash.wrapping_shr(16);
                        hash = hash.wrapping_mul(0x85ebca6b);
                        hash ^= hash.wrapping_shr(13);
                        hash = hash.wrapping_mul(0xc2b2ae35);
                        hash ^= hash.wrapping_shr(16);
                        return hash;
                    }
                    _ => panic!("Invalid read size!"),
                }
            }
            Err(e) => panic!(e),
        }
    }
}

