extern crate murmur3;

use std::io::Cursor;

#[test]
fn test1(){
	let string :&[u8] = "Lorem ipsum dolor sit amet, consectetur adipisicing elit".as_bytes();
	let mut tmp = Cursor::new(&string[0..string.len()]);
	assert!(murmur3::murmur3_32(& mut tmp, 0) == 0x3bf7e870)
}

#[test]
fn test2(){
	let string :&[u8] = "Hello, world!".as_bytes();
	let mut tmp = Cursor::new(&string[0..string.len()]);
	assert!(murmur3::murmur3_32(& mut tmp, 0) == 0xc0363e43)
}

#[test]
fn test3(){
	let string :&[u8] = "".as_bytes();
	let mut tmp = Cursor::new(&string[0..string.len()]);
	assert!(murmur3::murmur3_32(& mut tmp, 0) == 0x0)
}

#[test]
fn test4(){
	let string :&[u8] = "1".as_bytes();
	let mut tmp = Cursor::new(&string[0..string.len()]);
	assert!(murmur3::murmur3_32(& mut tmp, 0) == 0x9416ac93)	
}

#[test]
fn test5(){
	let string :&[u8] = "12".as_bytes();
	let mut tmp = Cursor::new(&string[0..string.len()]);
	assert!(murmur3::murmur3_32(& mut tmp, 0) == 0xf9d2ef15)	
}

#[test]
fn test6(){
	let string :&[u8] = "123".as_bytes();
	let mut tmp = Cursor::new(&string[0..string.len()]);
	assert!(murmur3::murmur3_32(& mut tmp, 0) == 0x9eb471eb)	
}

#[test]
fn test7(){
	let string :&[u8] = "1234".as_bytes();
	let mut tmp = Cursor::new(&string[0..string.len()]);
	assert!(murmur3::murmur3_32(& mut tmp, 0) == 0x721c5dc3)
}

#[test]
fn test8(){
	let string :&[u8] = "12345".as_bytes();
	let mut tmp = Cursor::new(&string[0..string.len()]);
	assert!(murmur3::murmur3_32(& mut tmp, 0) == 0x13a51193)	
}



