use osc94::Progress;
use std::{
    io::{Result, Write},
    thread::sleep,
    time::Duration,
};

fn work() {
    sleep(Duration::from_millis(50));
}

fn main() -> Result<()> {
    let mut stdout = std::io::stdout().lock();
    let mut progress = Progress::default();
    progress.start();

    for i in 0..=100 {
        work();
        write!(stdout, "{i}%\r")?;
        stdout.flush()?;
        progress.increment(1).flush()?;
    }

    Ok(())
}
