use std::hash::Hasher;
use byteorder::{LittleEndian, ByteOrder};
use std::io::Read;
use std::error::Error;

pub struct MurmurHasher{
    h1: u64,
    h2: u64,
    buf: [u8; 16],
    index: usize,
    processed: usize
}

impl Default for MurmurHasher {

    fn default() -> Self{
        MurmurHasher{
            h1: 0,
            h2: 0,
            buf: [0; 16],
            index: 0,
            processed: 0
        }
    }
}

pub fn murmur3_x64_128<T :Read>(source: &mut T, seed: u32) -> Result<u128, String> {
    let mut buffer:[u8;16] = [0; 16];
    let mut hasher = MurmurHasher::new(seed);
    loop {
        match source.read(&mut buffer) {
            Ok(16) => {
                hasher.write(&buffer);
            }
            Ok(0) => {
                return Ok(hasher.build_murmur_hash());
            }
            Err(e) => {
                return Err(String::from(e.description()))
            }
            Ok(i) => {
                hasher.write(&buffer[..i]);
                return Ok(hasher.build_murmur_hash());
            }
        }
    }
}

impl Hasher for MurmurHasher{
    fn finish(&self) -> u64 {
        self.build_murmur_hash() as u64
    }

    fn write(&mut self, bytes: &[u8]){
        self.processed += bytes.len();
        let to_split = if self.index == 0 {
            bytes
        }else{ 
            if bytes.len() + self.index >= 16 {
                let t = bytes.split_at(16 - self.index);
                for i in 0 .. (16 - self.index) {
                    self.buf[self.index + i] = t.0[i];
                }
                let r = process_16_bytes(self.h1, self.h2, &self.buf);
                self.h1 = r.0;
                self.h2 = r.1;
                self.index = 0;
                t.1
            }else{
                bytes
            }
        };
        for chunk in to_split.chunks(16) {
            if chunk.len() == 16{
                let t = process_16_bytes(self.h1, self.h2, chunk);
                self.h1 = t.0;
                self.h2 = t.1;
            }else{
                self.push_odd_bytes(chunk);
            }
        }
    }
}

impl MurmurHasher {
    pub fn new(seed:u32) -> Self{
        MurmurHasher{
            h1: seed as u64,
            h2: seed as u64,
            ..MurmurHasher::default()
        }
    }

    pub fn build_murmur_hash(&self) -> u128{
        let state = if self.index != 0 {
            process_odd_bytes(self.h1, self.h2, self.index, &self.buf)
        }else{
            (self.h1, self.h2)
        };
        finish(state.0, state.1, self.processed)
    }


    fn push_odd_bytes(&mut self, to_push: &[u8]){
        for x in to_push {
            self.buf[self.index] = *x;
            self.index += 1;
        }
    }
}


fn process_16_bytes(h1: u64, h2: u64, chunk:&[u8]) -> (u64,u64){
    const C1: u64 = 0x52dc_e729;
    const C2: u64 = 0x3849_5ab5;
    const R1: u32 = 27;
    const R2: u32 = 31;
    const M: u64 = 5;
    let mut h1 = h1;
    let mut h2 = h2;
    let k1 = LittleEndian::read_u64(&chunk[0..8]);
    let k2 = LittleEndian::read_u64(&chunk[8..]);
    h1 ^= process_h1_k_x64(k1);
    h1 = h1.rotate_left(R1).wrapping_add(h2).wrapping_mul(M).wrapping_add(C1);
    h2 ^= process_h2_k_x64(k2);
    h2 = h2.rotate_left(R2).wrapping_add(h1).wrapping_mul(M).wrapping_add(C2);
    (h1,h2)
}


fn process_odd_bytes(h1: u64, h2:u64, index: usize, buf:&[u8]) -> (u64,u64){
    match index {
        9...15 => {
            (h1 ^ process_h1_k_x64(LittleEndian::read_u64(&buf[0..8])),
            h2 ^ process_h2_k_x64(LittleEndian::read_uint(&buf[8..], index  - 8) as u64))
        }
        8 => {
            (h1 ^ process_h1_k_x64(LittleEndian::read_u64(&buf)), h2)
        }
        1...7 =>{
            (h1 ^ process_h1_k_x64(LittleEndian::read_uint(&buf, index ) as u64), h2)
        }
        _ => {
            panic!("Invalid index on process_odd_bytes");
        }
    }
}

#[inline(always)]
fn finish(h1: u64, h2:u64, processed: usize) -> u128 {
    let mut h1 = h1 ^ (processed as u64);
    let mut h2 = h2 ^ (processed as u64);
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    h1 = fmix64(h1);
    h2 = fmix64(h2);
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    ((h2 as u128) << 64) + (h1 as u128)
}



fn process_h1_k_x64(k: u64) -> u64 {
    const C1: u64 = 0x87c37b91114253d5;
    const C2: u64 = 0x4cf5ad432745937f;
    const R: u32 = 31;
    k.wrapping_mul(C1).rotate_left(R).wrapping_mul(C2)
}


fn process_h2_k_x64(k: u64) -> u64 {
    const C1: u64 = 0x87c37b91114253d5;
    const C2: u64 = 0x4cf5ad432745937f;
    const R: u32 = 33;
    k.wrapping_mul(C2).rotate_left(R).wrapping_mul(C1)
}


fn fmix64(k: u64) -> u64 {
    const C1: u64 = 0xff51_afd7_ed55_8ccd;
    const C2: u64 = 0xc4ce_b9fe_1a85_ec53;
    const R: u32 = 33;
    let mut tmp = k;
    tmp ^= tmp >> R;
    tmp = tmp.wrapping_mul(C1);
    tmp ^= tmp >> R;
    tmp = tmp.wrapping_mul(C2);
    tmp ^= tmp >> R;
    tmp
}