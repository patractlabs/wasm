//! Result
use core::result::Result as CoreResult;

/// Wasmx Errors
pub enum Error {
    /// Invalid Magic Number
    InvalidMagic,
    /// Io UnexpectedEof
    UnexpectedEof,
    /// Wrong wasm version
    UnSupportedVersion([u8; 4]),
}

/// Interpreter Result
pub type Result<T> = CoreResult<T, Error>;
