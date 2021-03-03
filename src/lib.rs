// Copyright (c) 2020 Stu Small
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! A pure rust implementation of the fast, non-cryptographic hash [murmur3](https://en.wikipedia.org/wiki/MurmurHash)
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
mod no_std;
#[cfg(not(feature = "std"))]
pub use no_std::{Cursor, Read, Result};

#[cfg(feature = "std")]
pub use std::io::{Cursor, Read, Result};

mod murmur3_32;
mod murmur3_x64_128;
mod murmur3_x86_128;

pub use self::murmur3_32::*;
pub use self::murmur3_x64_128::*;
pub use self::murmur3_x86_128::*;

fn copy_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Copy,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
    a
}
