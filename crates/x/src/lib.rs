//! WASM Interpreter
#![no_std]
#![warn(missing_docs)]
#![allow(unused_must_use)]
#![allow(dead_code)]

#[macro_use]
extern crate alloc;

pub mod cursor;
pub mod decoding;
pub mod result;
