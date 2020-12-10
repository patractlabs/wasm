//! WASM Module
use crate::{
    cursor::Cursor,
    decoding::{section::Section, Deserialize, MAGIC},
    result::{Error, Result},
};
use alloc::vec::Vec;

/// WASM Module
pub struct Module {
    magic: [u8; 4],
    version: [u8; 4],
    section: Vec<Section>,
}

impl Deserialize for Module {
    fn de(cursor: &mut Cursor) -> Result<Self> {
        // Check Magic
        let mut magic = [0; 4];
        cursor.read(&mut magic)?;
        if magic != MAGIC {
            return Err(Error::InvalidMagic);
        }

        // Deserialize Version
        let mut version = [0; 4];
        cursor.read(&mut version);
        if version[0] != 0x01 {
            return Err(Error::UnSupportedVersion(version));
        }

        // Check version
        // let mut sections = Vec::new();
        todo!()
    }
}
