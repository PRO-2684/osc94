# osc94

[![GitHub License](https://img.shields.io/github/license/PRO-2684/osc94?logo=opensourceinitiative)](https://github.com/PRO-2684/osc94/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/osc94/release.yml?logo=githubactions)](https://github.com/PRO-2684/osc94/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/osc94?logo=githubactions)](https://github.com/PRO-2684/osc94/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/osc94/total?logo=github)](https://github.com/PRO-2684/osc94/releases)
[![Crates.io Version](https://img.shields.io/crates/v/osc94?logo=rust)](https://crates.io/crates/osc94)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/osc94?logo=rust)](https://crates.io/crates/osc94)
[![docs.rs](https://img.shields.io/docsrs/osc94?logo=rust)](https://docs.rs/osc94)

Library for handling progress bar sequences (OSC 9;4).

## 💡 Example

```rust
use osc94::Progress;
use std::io::Result;

# fn work() {}
#
fn main() -> Result<()> {
    let mut progress = Progress::default();
    progress.start();

    for i in 0..=100 {
        work();
        progress.increment(1).flush()?;
    }

    Ok(())
}
```

## 📖 Usage

Usually, you'll only need the [`Progress`] struct, which provides a high-level interface and handles clean up job for you when dropped. If you need finer control, you can use the [`OSC94`] struct and [`ProgressState`] enum. See the documentation of respective API for more details.

## 📥 Binary

The provided binary allows you to play with progress bar sequences:

```shell
$ osc94
Sets the progress state and value of the progress bar for 1 second.

Usage: osc94 <state> <progress>

States:
  0/hidden
  1/normal
  2/error
  3/indeterminate
  4/warning

Progress: 0-100
```

It can be installed via the following methods:

### Using [`binstall`](https://github.com/cargo-bins/cargo-binstall)

```shell
cargo binstall osc94
```

### Downloading from Releases

Navigate to the [Releases page](https://github.com/PRO-2684/osc94/releases) and download respective binary for your platform. Make sure to give it execute permissions.

### Compiling from Source

```shell
cargo install osc94
```

## 🎉 Credits

- Microsoft Docs on [progress bar sequences](https://learn.microsoft.com/en-us/windows/terminal/tutorials/progress-bar-sequences) ([GitHub](https://github.com/MicrosoftDocs/terminal/blob/main/TerminalDocs/tutorials/progress-bar-sequences.md))
