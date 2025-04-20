//! This module is for handling progress bar sequence in a high-level way.

use super::{OSC94, ProgressState};
use std::io::{Result as IoResult, Stderr, Write};

/// A progress instance.
///
/// ## Examples
///
/// ```rust
/// use osc94::Progress;
///
/// # fn work() {}
/// #
/// let mut progress = Progress::default();
/// progress.start();
///
/// for i in 0..=100 {
///   work();
///   progress.increment(1);
///   progress.flush().unwrap();
/// }
/// ```
///
/// ## Usage
///
/// ### Creating a new progress bar
///
/// ```rust
/// # use osc94::Progress;
/// #
/// let mut progress = Progress::default(); // Writes to stderr by default
/// ```
///
/// To specify the output destination:
///
/// ```rust
/// # use osc94::Progress;
/// #
/// let mut progress = Progress::new(std::io::stdout()); // Writes to stdout instead
/// ```
///
/// ### Setting the state
///
/// By default, the progress bar is hidden. You can get and set it's state by calling [`get_state`](Progress::get_state) and [`state`](Progress::state()). Here's a list of all the states and convenience methods:
///
/// | State | Method(s) |
/// | ----- | --------- |
/// | [`Hidden`](ProgressState::Hidden) | [`hidden()`](Progress::hidden()) |
/// | [`Normal`](ProgressState::Normal) | [`normal()`](Progress::normal()), [`start()`](Progress::start()) |
/// | [`Error`](ProgressState::Error) | [`error()`](Progress::error()) |
/// | [`Indeterminate`](ProgressState::Indeterminate) | [`indeterminate()`](Progress::indeterminate()) |
/// | [`Warning`](ProgressState::Warning) | [`warning()`](Progress::warning()) |
///
/// ### Setting the progress
///
/// You can get and set the progress value by calling [`get_progress`](Progress::get_progress) and [`progress`](Progress::progress()). You can also increment the progress value by calling [`increment`](Progress::increment()). Both `progress` and `increment` will saturate at 100. To check if the progress bar is finished, you can call [`is_finished`](Progress::is_finished()).
///
/// ### Flushing
///
/// You must call [`flush`](Progress::flush()) to update the progress bar. This will write the current state and progress value to the destination. The [default](Progress::default()) destination is `stderr`, but you can specify a different destination when creating the progress bar with [`new`](Progress::new()).
///
/// ### Resetting
///
/// When the `Progress` instance is dropped, it will automatically reset the progress bar to the default state (i.e. hides the progress bar).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Progress<W: Write> {
    /// The state of the progress bar.
    state: ProgressState,
    /// The progress value (0-100).
    progress: u8,
    /// The writing destination of the progress bar.
    destination: W,
}

impl<W: Write> Progress<W> {
    // General methods for creating and flushing the progress bar.

    /// Creates a new `Progress` instance.
    pub const fn new(destination: W) -> Self {
        Self {
            state: ProgressState::Hidden,
            progress: 0,
            destination,
        }
    }

    /// Flushes the progress bar.
    ///
    /// # Errors
    ///
    /// This function will return an error if IO operations fail.
    pub fn flush(&mut self) -> IoResult<&mut Self> {
        let raw = OSC94 {
            state: self.state,
            progress: self.progress,
        };
        write!(self.destination, "{raw}")?;

        Ok(self)
    }

    // State-related methods

    /// Gets the current state of the progress bar.
    pub const fn get_state(&self) -> ProgressState {
        self.state
    }

    /// Sets the state of the progress bar.
    pub const fn state(&mut self, state: ProgressState) -> &mut Self {
        self.state = state;
        self
    }

    /// Starts the progress bar in normal state. (Equivalent to [`normal`](Progress::normal))
    pub const fn start(&mut self) -> &mut Self {
        self.normal()
    }

    /// Sets the state of the progress bar to [`Hidden`](ProgressState::Hidden).
    pub const fn hidden(&mut self) -> &mut Self {
        self.state(ProgressState::Hidden)
    }

    /// Sets the state of the progress bar to [`Normal`](ProgressState::Normal).
    pub const fn normal(&mut self) -> &mut Self {
        self.state(ProgressState::Normal)
    }

    /// Sets the state of the progress bar to [`Error`](ProgressState::Error).
    pub const fn error(&mut self) -> &mut Self {
        self.state(ProgressState::Error)
    }

    /// Sets the state of the progress bar to [`Indeterminate`](ProgressState::Indeterminate).
    pub const fn indeterminate(&mut self) -> &mut Self {
        self.state(ProgressState::Indeterminate)
    }

    /// Sets the state of the progress bar to [`Warning`](ProgressState::Warning).
    pub const fn warning(&mut self) -> &mut Self {
        self.state(ProgressState::Warning)
    }

    // Progress-related methods

    /// Gets the current progress value.
    pub const fn get_progress(&self) -> u8 {
        self.progress
    }

    /// Sets the progress value, saturating at 100.
    pub const fn progress(&mut self, progress: u8) -> &mut Self {
        // self.progress = progress.min(100); // Not `const`
        self.progress = if progress > 100 { 100 } else { progress };
        self
    }

    /// Increments the progress value, saturating at 100.
    pub const fn increment(&mut self, by: u8) -> &mut Self {
        self.progress(self.progress.saturating_add(by));
        self
    }

    /// Whether the progress bar is finished.
    pub const fn is_finished(&self) -> bool {
        self.progress == 100
    }
}

impl<W: Write> Drop for Progress<W> {
    /// Resets the progress bar to the default state (hidden) when dropped.
    fn drop(&mut self) {
        let raw = OSC94::default();
        let _ = write!(self.destination, "{raw}");
    }
}

impl Default for Progress<Stderr> {
    /// Creates a new `DefaultProgress` instance with default values.
    fn default() -> Self {
        Self {
            state: ProgressState::Hidden,
            progress: 0,
            destination: std::io::stderr(),
        }
    }
}
