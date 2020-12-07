//! WASM Section
mod code;
mod custom;
mod element;
mod export;
mod function;
mod global;
mod import;
mod memory;
mod name;
mod reloc;
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
    Import(import::Section),
    /// Function Section
    Function(function::Section),
    /// Table Section
    Table(table::Section),
    /// Memory Section
    Memory(memory::Section),
    /// Global Section
    Global(global::Section),
    /// Export Section
    Export(export::Section),
    /// Entry reference of the module
    Start(u32),
    /// Elements section
    Element(element::Section),
    /// Number of passive data entries
    DataCount(u32),
    /// Code Section
    Code(code::Section),
    /// Name Section
    Name(name::Section),
    /// Relocation Section
    Reloc(reloc::Section),
}
