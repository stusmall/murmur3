// Copyright (c) 2020 Stu Small
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

extern crate byteorder;
#[macro_use]
extern crate quickcheck;
extern crate murmur3;
extern crate murmur3_sys;

use std::io::Cursor;

use byteorder::{ByteOrder, LittleEndian};

use murmur3::murmur3_32;
use murmur3_sys::MurmurHash3_x86_32;

use murmur3::murmur3_x86_128;
use murmur3_sys::MurmurHash3_x86_128;

use murmur3::murmur3_x64_128;
use murmur3_sys::MurmurHash3_x64_128;

quickcheck! {
    fn quickcheck_32(input:(u32, Vec<u8>)) -> bool{
        let seed = input.0;
        let xs = input.1;
        let output: [u8; 4] = [0; 4];
        unsafe {
            MurmurHash3_x86_32(xs.as_ptr() as _, xs.len() as i32,seed,output.as_ptr() as *mut _)
        };
        let output = LittleEndian::read_u32(&output);
        let output2 = murmur3_32(&mut Cursor::new(xs), seed).unwrap();
        output == output2
    }
}

quickcheck! {
    fn quickcheck_x86_128(input:(u32, Vec<u8>)) -> bool {
        let seed = input.0;
        let xs = input.1;
        let output_bytes: [u8; 16] = [0; 16];
        unsafe {
            MurmurHash3_x86_128(xs.as_ptr() as _, xs.len() as i32,seed,output_bytes.as_ptr() as *mut _)
        };
        let output = LittleEndian::read_u128(&output_bytes);
        let output2 = murmur3_x86_128(&mut Cursor::new(xs), seed).unwrap();
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
