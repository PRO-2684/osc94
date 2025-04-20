use std::io::Result;

use osc94::{OSC94, Progress, ProgressState};

#[test]
fn test_raw() {
    let mut progress = OSC94::new();
    let states = [
        ProgressState::Hidden,
        ProgressState::Normal,
        ProgressState::Error,
        ProgressState::Indeterminate,
        ProgressState::Warning,
    ];

    for state in states {
        progress.state = state;

        for i in 0..=100 {
            progress.progress = i;
            let state_val = state as u8;
            assert_eq!(
                progress.to_string(),
                format!("\x1b]9;4;{state_val};{i}\x07")
            );
        }
    }
}

#[test]
fn test_progress() {
    // Initialization
    let sink = std::io::sink(); // Discard output
    let mut progress = Progress::new(sink);
    assert_eq!(progress.get_state(), ProgressState::Hidden);
    assert_eq!(progress.get_progress(), 0);

    // Test state changes
    progress.start();
    assert_eq!(progress.get_state(), ProgressState::Normal);
    progress.hidden();
    assert_eq!(progress.get_state(), ProgressState::Hidden);
    progress.error();
    assert_eq!(progress.get_state(), ProgressState::Error);
    progress.indeterminate();
    assert_eq!(progress.get_state(), ProgressState::Indeterminate);
    progress.warning();
    assert_eq!(progress.get_state(), ProgressState::Warning);
    progress.normal();
    assert_eq!(progress.get_state(), ProgressState::Normal);

    // Test progress updates
    progress.progress(150);
    assert_eq!(progress.get_progress(), 100);
    progress.progress(50);
    assert_eq!(progress.get_progress(), 50);
    assert!(!progress.is_finished());
    progress.increment(51);
    assert_eq!(progress.get_progress(), 100);
    assert!(progress.is_finished());
}

#[test]
fn test_progress_repr() -> Result<()> {
    let mut buffer = Vec::new();

    {
        let mut progress = Progress::new(&mut buffer);
        progress.state(ProgressState::Normal).progress(50).flush()?;
        assert_eq!(progress.get_progress(), 50);
    }

    let output = String::from_utf8(buffer).expect("Failed to convert buffer to string");
    let expected_1 = OSC94 {
        state: ProgressState::Normal,
        progress: 50,
    };
    let expected_2 = OSC94::default(); // Should be flushed when dropped

    assert_eq!(output, format!("{expected_1}{expected_2}"));

    Ok(())
}
