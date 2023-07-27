//! This library provides the ability to read Unchained Index chunk files.
//!
//! The chunk files map Ethereum addresses to the transactions they appear in.
//! Functions in this library allow for this data to be extracted for use.
pub(crate) mod constants;
pub mod files;
pub mod parse;
pub mod structure;

pub use parse::*;
