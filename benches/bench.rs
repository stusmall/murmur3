
#![feature(test)]

extern crate test;
use test::Bencher;
use std::io::Cursor;

extern crate murmur3;

#[bench]
fn bench1(b: &mut Bencher) {
    b.iter(|| {
        let string: &[u8] = "Lorem ipsum dolor sit amet, consectetur adipisicing elit".as_bytes();
        let mut tmp = Cursor::new(&string[0..string.len()]);
        murmur3::murmur3_32(& mut tmp, 0);
    });
}

