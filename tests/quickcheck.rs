extern crate byteorder;
#[macro_use]
extern crate quickcheck;
extern crate murmur3;
extern crate murmur3_sys;

use std::io::Cursor;
use std::hash::Hasher;

use byteorder::{LittleEndian, ByteOrder};

use murmur3::murmur3_32::MurmurHasher as MurmurHasher32;
use murmur3_sys::MurmurHash3_x86_32;

use murmur3::murmur3_x86_128;
use murmur3_sys::MurmurHash3_x86_128;

use murmur3::murmur3_x64_128::murmur3_x64_128;
//use murmur3::murmur3_x64_128::MurmurHasher as MurmurHasher_64_128;
use murmur3_sys::MurmurHash3_x64_128;



quickcheck!{
    fn quickcheck_32(input:(u32, Vec<u8>)) -> bool{
        let seed = input.0;
        let xs = input.1;
        let output: [u8; 4] = [0; 4];
        unsafe {
            MurmurHash3_x86_32(xs.as_ptr() as _, xs.len() as i32,seed,output.as_ptr() as *mut _)
        };
        let output = LittleEndian::read_u32(&output);
        let mut hasher = MurmurHasher32::new(seed);
        hasher.write(&xs);
        let output2 = hasher.build_murmur_hash();
        output == output2
    }
}


quickcheck! {
    fn quickcheck_x86_128(input:(u32, Vec<u8>)) -> bool {
        let seed = input.0;
        let xs = input.1;
        let output: [u8; 16] = [0; 16];
        unsafe {
            MurmurHash3_x86_128(xs.as_ptr() as _, xs.len() as i32,seed,output.as_ptr() as *mut _)
        };
        let mut output2: [u8; 16] = [0; 16];
        murmur3_x86_128(&mut Cursor::new(xs), seed, &mut output2);
        output == output2
    }
}

quickcheck! {
    fn quickcheck_x64_128(input:(u32, Vec<u8>)) -> bool {
        let seed = input.0;
        let xs = input.1;
        let output_bytes: [u8; 16] = [0; 16];
        unsafe {
            MurmurHash3_x64_128(xs.as_ptr() as _, xs.len() as i32,seed, output_bytes.as_ptr() as *mut _)
        };
        let output = LittleEndian::read_u128(&output_bytes);
        let output2 = murmur3_x64_128(&mut Cursor::new(xs), seed).unwrap();
        output == output2
    }
}