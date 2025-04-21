use osc94::Progress;
use std::{io::Result, thread::sleep, time::Duration};

fn main() -> Result<()> {
    let mut progress = Progress::default();
    progress.indeterminate().flush()?;

    sleep(Duration::from_secs(1));

    Ok(())
}
