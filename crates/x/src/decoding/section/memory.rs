use crate::decoding::ty::ResizableLimit;
use alloc::vec::Vec;

/// Memory Section
pub struct Section(Vec<Memory>);

/// Memory Entry
pub struct Memory(ResizableLimit);
