extern crate byteorder;
#[macro_use]
extern crate quickcheck;
extern crate murmur3;
extern crate murmur3_sys;

use std::io::Cursor;

use byteorder::{LittleEndian, ByteOrder};

use murmur3::murmur3_32;
use murmur3_sys::MurmurHash3_x86_32;

use murmur3::murmur3_x64_128;
use murmur3_sys::MurmurHash3_x64_128;

use murmur3::murmur3_x86_128;
use murmur3_sys::MurmurHash3_x86_128;

quickcheck!{
    fn quickcheck_32(xs: Vec<u8>) -> bool{
        let output: [u8; 4] = [0; 4];
        unsafe {
            MurmurHash3_x86_32(xs.as_ptr() as _, xs.len() as i32,0,output.as_ptr() as *mut _)
        };
        let output = LittleEndian::read_u32(&output);
        let output2 = murmur3_32(&mut Cursor::new(xs), 0);
        output == output2
    }
}


quickcheck! {
    fn quickcheck_x86_128(xs: Vec<u8>) -> bool {
        let output: [u8; 16] = [0; 16];
        unsafe {
            MurmurHash3_x86_128(xs.as_ptr() as _, xs.len() as i32,0,output.as_ptr() as *mut _)
        };
        let mut output2: [u8; 16] = [0; 16];
        murmur3_x86_128(&mut Cursor::new(xs), 0, &mut output2);
        output == output2
    }
}

quickcheck! {
    fn quickcheck_x64_128(xs: Vec<u8>) -> bool {
        let output: [u8; 16] = [0; 16];
        unsafe {
            MurmurHash3_x64_128(xs.as_ptr() as _, xs.len() as i32,0,output.as_ptr() as *mut _)
        };
        let mut output2: [u8; 16] = [0; 16];
        murmur3_x64_128(&mut Cursor::new(xs), 0, &mut output2);
        output == output2
    }
}