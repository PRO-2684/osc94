//! This module is for handling states of progress bar sequence (OSC 9;4).

/// Possible states of a progress bar.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressState {
    /// The default state, indicating that the progress bar should be hidden.
    ///
    /// Use this state when the command is complete, to clear out any progress state.
    #[default]
    Hidden = 0,
    /// Set progress value to [`progress`](ProgressBar::progress), in the "Normal" state.
    Normal = 1,
    /// Set progress value to [`progress`](ProgressBar::progress), in the "Error" state.
    Error = 2,
    /// Set the taskbar to the "Indeterminate" state, ignoring the [`progress`](ProgressBar::progress) value.
    ///
    /// This is useful for commands that don't have a progress value, but are still running.
    Indeterminate = 3,
    /// Set progress value to [`progress`](ProgressBar::progress), in the "Warning" state.
    Warning = 4,
}
