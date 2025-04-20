//! # `osc94` library crate
//!
//! Library for handling progress bar sequences (OSC 9;4).

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

pub mod raw;
mod state;

pub use raw::OSC94;
pub use state::ProgressState;
