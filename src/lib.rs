#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

#[cfg(test)]
extern crate std;

#[cfg(feature = "std")]
mod progress;
pub mod raw;
mod state;

#[cfg(feature = "std")]
pub use progress::Progress;
pub use raw::OSC94;
pub use state::ProgressState;
