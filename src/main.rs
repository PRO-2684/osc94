#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use osc94::{Progress, ProgressState};
use std::{io::{Error, ErrorKind, Result}, thread::sleep, time::Duration};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().take(3).collect();
    if args.len() != 3 {
        eprintln!("Sets the progress state and value of the progress bar for 1 second.\n");
        eprintln!("Usage: {} <state> <progress>\n", args[0]);
        eprintln!("States:\n  0/hidden\n  1/normal\n  2/error\n  3/indeterminate\n  4/warning\n");
        eprintln!("Progress: 0-100\n");
        return Ok(());
    }

    let Some(state) = interpret_state(&args[1]) else {
        eprintln!("Invalid state: {}", args[1]);
        return Err(error("Invalid state"));
    };

    let progress_value: u8 = match args[2].parse() {
        Ok(value) if value <= 100 => value,
        _ => {
            eprintln!("Invalid progress value: {}", args[2]);
            return Err(error("Invalid progress value"));
        }
    };

    let mut progress = Progress::default();
    progress.state(state)
        .progress(progress_value)
        .flush()?;

    sleep(Duration::from_secs(1));

    Ok(())
}

/// Try to interpret `ProgressState` from a string.
fn interpret_state(state: &str) -> Option<ProgressState> {
    match state.to_lowercase().as_str() {
        "0" | "hidden" => Some(ProgressState::Hidden),
        "1" | "normal" => Some(ProgressState::Normal),
        "2" | "error" => Some(ProgressState::Error),
        "3" | "indeterminate" => Some(ProgressState::Indeterminate),
        "4" | "warning" => Some(ProgressState::Warning),
        _ => None,
    }
}

/// Construct a custom error.
fn error(msg: &str) -> Error {
    Error::new(ErrorKind::InvalidInput, msg)
}
