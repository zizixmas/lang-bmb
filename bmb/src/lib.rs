//! BMB Compiler Library
//!
//! AI-Native programming language with contract-based verification.

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod smt;
pub mod types;
pub mod verify;

pub use ast::Span;
pub use error::{CompileError, Result};
