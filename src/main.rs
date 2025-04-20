#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use osc94::Progress;
use std::{io::Result, thread::sleep, time::Duration};

fn main() -> Result<()> {
    let duration = Duration::from_millis(50);
    let mut progress = Progress::default();
    progress.start();

    for i in 0..=100 {
        progress.progress(i).flush()?;
        eprintln!("{i}%");
        sleep(duration);
    }
    eprintln!("Success.");

    Ok(())
}
