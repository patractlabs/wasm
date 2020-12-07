//! Import Section
use crate::decoding::ty::{Table, Value};
use alloc::{string::String, vec::Vec};

/// Import Section
pub struct Section(Vec<ImportEntry>);

/// Import Entry
pub struct ImportEntry {
    module: String,
    field: String,
    external: External,
}

/// External to local bindings
pub enum External {
    Function(u32),
    Table(Table),
    Memory,
    Global,
}

/// Global Type
pub struct Global {
    ty: Value,
    is_mutable: bool,
}
