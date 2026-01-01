//! Contract verification module
//!
//! Verifies function contracts (pre/post conditions) using SMT solving.

mod contract;

pub use contract::{ContractVerifier, VerificationReport, FunctionReport};
