//! Types
use alloc::vec::Vec;

/// Block Type
pub enum Block {
    /// Specfied block type
    Value(Value),
    /// No specfied block type
    NoResult,
}

/// Value Type
pub enum Value {
    /// 32-bit signed integer
    I32,
    /// 64-bit signed integer
    I64,
    /// 32-bit float
    F32,
    /// 64-bit float
    F64,
}

/// Function Type
pub struct Function {
    from: u8,
    param: Vec<Value>,
    result: Vec<Value>,
}

/// Global Type
pub struct Global {
    value: Value,
    is_mutable: bool,
}

/// Table Type
pub struct Table {
    element: Element,
    limit: ResizableLimit,
}

/// Element Type
pub enum Element {
    AnyFunc,
}

/// Memory and table limits
pub struct ResizableLimit {
    initial: u32,
    maximum: Option<u32>,
}
