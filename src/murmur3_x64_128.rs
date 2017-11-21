use std::hash::Hasher;
use byteorder::{LittleEndian, ByteOrder};

pub struct MurmurHasher{
    h1: u64,
    h2: u64,
    buf: [u8; 16],
    index: usize,
    processed: u32
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

impl Hasher for MurmurHasher{
    fn write(&mut self, bytes: &[u8]){
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

    fn finish(&self) -> u64 {
        let state =if self.index != 0 {
            process_odd_bytes(self.h1, self.h2, self.index, &self.buf)
        }else{
            (self.h1, self.h2)
        };

        let x = finish(state.0, state.1, self.processed);
        //This hasher wants a 64 bit return even if we have 128 bits.
        //I'm just going to XOR the two 64 bit chunks together to get
        //what they want.
        x.0 ^ x.1
    }
}

impl MurmurHasher {
    fn push_odd_bytes(&mut self, to_push: &[u8]){
        for x in to_push {
            self.buf[self.index] = *x;
            self.index += 1;
        }
    }
}


fn process_16_bytes(h1: u64, h2: u64, chunk:&[u8]) -> (u64,u64){
    const C1: u64 = 0x52dce729;
    const C2: u64 = 0x38495ab5;
    const R1: u32 = 27;
    const R2: u32 = 31;
    const M: u64 = 5;
    let mut h1 = h1;
    let mut h2 = h2;
    let (t1,t2) = chunk.split_at(8);
    let k1 = LittleEndian::read_u64(t1);
    let k2 = LittleEndian::read_u64(t2);
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
            h2 ^ process_h2_k_x64(LittleEndian::read_int(&buf[8..], index - 1 - 8) as u64))
        }
        8 => {
            (h1 ^ process_h1_k_x64(LittleEndian::read_u64(&buf)), h2)
        }
        1...7 =>{
            (h1 ^ process_h1_k_x64(LittleEndian::read_int(&buf, index - 1) as u64), h2)
        }
        _ => {
            panic!("Invalid index on process_odd_bytes");
        }
    }
}


fn finish(h1: u64, h2:u64, processed: u32) -> (u64,u64) {
    let mut h1 = h1 ^ (processed as u64);
    let mut h2 = h2 ^ (processed as u64);
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    h1 = fmix64(h1);
    h2 = fmix64(h2);
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    (h1,h2)
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
    const C1: u64 = 0xff51afd7ed558ccd;
    const C2: u64 = 0xc4ceb9fe1a85ec53;
    const R: u32 = 33;
    let mut tmp = k;
    tmp ^= tmp >> R;
    tmp = tmp.wrapping_mul(C1);
    tmp ^= tmp >> R;
    tmp = tmp.wrapping_mul(C2);
    tmp ^= tmp >> R;
    tmp
}