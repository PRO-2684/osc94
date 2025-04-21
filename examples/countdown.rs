use osc94::Progress;
use std::{io::Result, thread::sleep, time::Duration};

fn main() -> Result<()> {
    let mut progress = Progress::default();
    let mut value = 100;
    progress.warning().progress(value).flush()?;

    for _ in 0..10 {
        value -= 10;
        progress.progress(value).flush()?;
        sleep(Duration::from_secs(1));
    }

    Ok(())
}

