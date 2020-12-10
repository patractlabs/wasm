//! Function
use crate::decoding::{instr::Instruction, ty::Value};
use alloc::vec::Vec;

/// Function Signature
pub struct Func(u32);

/// Local definition inside the function body.
pub struct Local {
    count: u32,
    value_type: Value,
}

/// Function body definition.
pub struct FuncBody {
    locals: Vec<Local>,
    instructions: Vec<Instruction>,
}
