//! WASM Section
mod custom;
mod global;
mod import;
mod memory;
mod table;
mod ty;

use alloc::vec::Vec;

/// WASM Section
pub enum Section {
    /// Unparsed section
    Unparsed {
        /// Unparsed section id
        id: u8,
        /// raw bytes of the section
        payload: Vec<u8>,
    },
    /// Custom Section
    Custom(custom::Section),
    /// Type Section
    Type(ty::Section),
    /// Import Section
    Import(Vec<u8>),
    /// Function Section
    Function(Vec<u8>),
    /// Table Section
    Table(table::Section),
    /// Memory Section
    Memory(memory::Section),
    /// Global Section
    Global(Vec<u8>),
    /// Export Section
    Export(Vec<u8>),
    /// Entry reference of the module
    Start(u32),
    /// Elements section
    Element(Vec<u8>),
    /// Number of passive data entries
    DataCount(u32),
    /// Function Section
    Code(Vec<u8>),
    /// Name Section
    Name(Vec<u8>),
    /// Relocation Section
    Reloc(Vec<u8>),
}
