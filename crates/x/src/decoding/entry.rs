//! WASM Entries
use crate::decoding::{
    instr::Instruction,
    ty::{Global as GlobalType, Table},
};
use alloc::{string::String, vec::Vec};

/// Import Entry
pub struct Import {
    module: String,
    field: String,
    external: External,
}

/// External to local bindings
pub enum External {
    Function(u32),
    Table(Table),
    Memory,
    Global(GlobalType),
}

/// Global Entry
pub struct Global {
    /// Global Type
    global: GlobalType,
    /// Init Expr
    init: Vec<Instruction>,
}

/// Export Entry
pub struct Export {
    field: String,
    internal: Internal,
}

/// Internal reference of the exported entry
pub enum Internal {
    /// Function reference.
    Function(u32),
    /// Table reference.
    Table(u32),
    /// Memory reference.
    Memory(u32),
    /// Global reference.
    Global(u32),
}
