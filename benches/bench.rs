
#![feature(test)]

extern crate test;
use test::Bencher;
use std::io::Cursor;
use std::hash::Hasher;

extern crate murmur3;

use murmur3::murmur3_32;
use murmur3::murmur3_x64_128;

#[bench]
fn bench_32(b: &mut Bencher) {
    let string: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    b.bytes = string.len() as u64;
    b.iter(|| {
        let mut tmp = Cursor::new(&string[0..string.len()]);
        murmur3::murmur3_32(&mut tmp, 0)
    });
}

#[bench]
fn new_bench_32(b: &mut Bencher) {
    let string: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    b.bytes = string.len() as u64;
    b.iter(|| {
        let mut h = murmur3_32::MurmurHasher::default();
        h.write(string);
        h.finish()
    });
}

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
fn new_bench_32_100(b: &mut Bencher) {
    let string: &[u8] = &[0; 200];
    b.bytes = string.len() as u64;
    b.iter(|| {
        let mut h = murmur3_32::MurmurHasher::default();
        h.write(string);
        h.finish()
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



