//! Interpreter module for BMB

mod env;
mod error;
mod eval;
mod value;

pub use env::{child_env, EnvRef, Environment};
pub use error::{ErrorKind, InterpResult, RuntimeError};
pub use eval::{BuiltinFn, Interpreter};
pub use value::Value;
