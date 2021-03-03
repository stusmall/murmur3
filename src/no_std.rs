//! This file replaces some of std::io in a no_std environment

use alloc::vec::Vec;
use core::cmp::min;

/// The error
#[derive(Debug)]
pub enum Error {}

/// The result of reading
pub type Result<T> = core::result::Result<T, Error>;

/// A generic reading trait
pub trait Read {
    /// Read into a buffer.
    /// @return the number of bytes read into the buffer
    fn read<const N: usize>(&mut self, buffer: &mut [u8; N]) -> Result<usize>;
}

/// A Cursor
pub struct Cursor<T> {
    inner: T,
    pos: usize,
}

impl<T> Cursor<T> {
    /// Create a new cursor
    pub fn new(inner: T) -> Self {
        Self { inner, pos: 0 }
    }
}

impl Read for Cursor<&[u8]> {
    fn read<const N: usize>(&mut self, buffer: &mut [u8; N]) -> Result<usize> {
        // get number of items we can copy
        let n_items = min(self.inner.len() - self.pos, N);

        // split the buffers to make sure
        let (left_buffer, _) = buffer.split_at_mut(n_items);
        let (left_inner, _) = self.inner[self.pos..].split_at(n_items);

        // copy items from inner to buffer
        left_buffer.copy_from_slice(left_inner);
        self.pos += n_items;

        Result::Ok(n_items)
    }
}

impl Read for Cursor<Vec<u8>> {
    fn read<const N: usize>(&mut self, buffer: &mut [u8; N]) -> Result<usize> {
        // get number of items we can copy
        let n_items = min(self.inner.len() - self.pos, N);

        // split the buffers to make sure
        let (left_buffer, _) = buffer.split_at_mut(n_items);
        let (left_inner, _) = self.inner[self.pos..].split_at(n_items);

        // copy items from inner to buffer
        left_buffer.copy_from_slice(left_inner);
        self.pos += n_items;

        Result::Ok(n_items)
    }
}

impl Read for Cursor<&str> {
    fn read<const N: usize>(&mut self, buffer: &mut [u8; N]) -> Result<usize> {
        // get number of items we can copy
        let n_items = min(self.inner.len() - self.pos, N);

        // split the buffers to make sure
        let (left_buffer, _) = buffer.split_at_mut(n_items);
        let (left_inner, _) = self.inner[self.pos..].split_at(n_items);

        // copy items from inner to buffer
        left_buffer.copy_from_slice(left_inner.as_bytes());
        self.pos += n_items;

        Result::Ok(n_items)
    }
}
