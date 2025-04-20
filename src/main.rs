#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use osc94::{OSC94, ProgressState};
use std::{thread::sleep, time::Duration};

fn main() {
    let duration = Duration::from_millis(50);
    let mut progress = OSC94::new();
    progress.state = ProgressState::Default;

    for i in 0..=100 {
        progress.progress = i;
        println!("{i}%{progress}");
        sleep(duration);
    }

    progress.state = ProgressState::Hidden;
    println!("Success.{progress}");
}
