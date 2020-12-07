use crate::decoding::instr::InitExpr;
use alloc::vec::Vec;

/// Entry in the element section
pub struct ElementSegment {
    index: u32,
    offset: Option<InitExpr>,
    members: Vec<u32>,
}
