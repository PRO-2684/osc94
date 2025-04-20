//! This module is for handling raw progress bar sequence (OSC 9;4).

use std::fmt::Display;
use super::ProgressState;

/// A raw progress bar sequence (OSC 9;4).
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OSC94 {
    /// The state of the progress bar.
    pub state: ProgressState,
    /// The progress value (0-100).
    pub progress: u8,
}

impl OSC94 {
    /// Creates a default progress bar sequence.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Display for OSC94 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b]9;4;{};{}\x07", self.state as u8, self.progress)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_progress_sequence() {
        let sequence = OSC94::new();
        assert_eq!(format!("{}", sequence), "\x1b]9;4;0;0\x07");
    }
}
