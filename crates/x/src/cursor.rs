//! Byte Cursor
use crate::result::{Error, Result};

/// The Cursor
pub struct Cursor<'s> {
    inner: &'s [u8],
    pos: usize,
}

impl<'s> Cursor<'s> {
    /// Fill the requested bytes
    pub fn read(&self, buf: &mut [u8]) -> Result<()> {
        let remainder = self.inner.len() - self.pos;
        let requested = buf.len();
        if requested > remainder {
            return Err(Error::UnexpectedEof);
        }

        buf.copy_from_slice(&self.inner[self.pos..(self.pos + requested)]);
        Ok(())
    }
}
