
#![feature(test)]

extern crate test;
extern crate murmur3_sys;
use test::Bencher;
use std::io::Cursor;
use std::hash::Hasher;

extern crate murmur3;

use murmur3::murmur3_32;
use murmur3::murmur3_x64_128;

use murmur3_sys::MurmurHash3_x86_32;




#[bench]
fn new_bench_x64_128(b: &mut Bencher) {
    let string: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    b.bytes = string.len() as u64;
    b.iter(|| {
        let mut h = murmur3_x64_128::MurmurHasher::default();
        h.write(string);
        h.finish()
    });
}

#[bench]
fn bench_32(b: &mut Bencher) {
    let string: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    b.bytes = string.len() as u64;
    b.iter(|| {
        let mut h = murmur3_32::MurmurHasher::default();
        h.write(string);
        h.finish()
    });
}

#[bench]
fn bench_c_32(b: &mut Bencher) {
    let string: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    b.bytes = string.len() as u64;
    b.iter(|| {
        unsafe {
            let output: [u8; 4] = [0; 4];
            MurmurHash3_x86_32(string.as_ptr() as _, string.len() as i32,0,output.as_ptr() as *mut _)
        };
    });
}

#[bench]
fn bench_x86_128(b: &mut Bencher) {
    let string: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    b.bytes = string.len() as u64;
    b.iter(|| {
        let mut out: [u8; 16] = [0; 16];
        let mut tmp = Cursor::new(&string[0..string.len()]);
        murmur3::murmur3_x86_128(&mut tmp, 0, &mut out);
    });
}

#[bench]
fn bench_x64_128(b: &mut Bencher) {
    let string: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    b.bytes = string.len() as u64;
    b.iter(|| {
        let mut out: [u8; 16] = [0; 16];
        let mut tmp = Cursor::new(&string[0..string.len()]);
        murmur3::murmur3_x64_128(&mut tmp, 0, &mut out);
    });
}






#[bench]
fn bench_c_x64_128(b: &mut Bencher) {
    let string: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    b.bytes = string.len() as u64;
    b.iter(|| {
        unsafe{
            let output: [u8; 16] = [0; 16];
            murmur3_sys::MurmurHash3_x64_128(string.as_ptr() as _,string.len() as i32,0,output.as_ptr() as *mut _);
        }
    });
}