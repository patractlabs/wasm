//! Global Section
use crate::decoding::{instr::Instruction, ty::Global};
use alloc::vec::Vec;

/// Global Section
pub struct Section;

/// Global Entry
pub struct GlobalEntry {
    /// Global Type
    global: Global,
    /// Init Expr
    init: Vec<Instruction>,
}
