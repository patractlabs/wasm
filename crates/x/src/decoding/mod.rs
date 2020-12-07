//! Decoding

/// Magic Number
pub const MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];

mod entry;
mod func;
mod instr;
mod module;
mod section;
mod segment;
mod ty;

use crate::{cursor::Cursor, result::Result};

/// Deserialize WASM module
pub trait Deserialize: Sized {
    /// Deserialize struct from bytes
    fn de(cursor: &mut Cursor) -> Result<Self>;
}
