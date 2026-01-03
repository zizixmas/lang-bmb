//! LLVM Code Generation
//!
//! This module generates LLVM IR from MIR.
//!
//! Two backends are available:
//! - `llvm` feature: Uses inkwell (LLVM C API) for direct compilation
//! - Default: Uses text-based LLVM IR generation + external clang

mod llvm_text;
pub use llvm_text::{TextCodeGen, TextCodeGenError, TextCodeGenResult};

#[cfg(feature = "llvm")]
mod llvm;

#[cfg(feature = "llvm")]
pub use llvm::*;

#[cfg(not(feature = "llvm"))]
mod stub;

#[cfg(not(feature = "llvm"))]
pub use stub::*;
