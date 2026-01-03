//! Code Generation Backends
//!
//! This module generates executable code from MIR.
//!
//! Available backends:
//! - LLVM (native): Uses text-based LLVM IR generation + external clang
//! - WASM (portable): Uses text-based WAT generation + wat2wasm
//!
//! ```text
//! MIR (Middle IR)
//!     ├── llvm_text.rs → .ll → clang → Native Binary
//!     └── wasm_text.rs → .wat → wat2wasm → .wasm
//! ```

mod llvm_text;
mod wasm_text;

pub use llvm_text::{TextCodeGen, TextCodeGenError, TextCodeGenResult};
pub use wasm_text::{WasmCodeGen, WasmCodeGenError, WasmCodeGenResult, WasmTarget};

#[cfg(feature = "llvm")]
mod llvm;

#[cfg(feature = "llvm")]
pub use llvm::*;

#[cfg(not(feature = "llvm"))]
mod stub;

#[cfg(not(feature = "llvm"))]
pub use stub::*;
