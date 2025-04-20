# osc94

[![GitHub License](https://img.shields.io/github/license/PRO-2684/osc94?logo=opensourceinitiative)](https://github.com/PRO-2684/osc94/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/osc94/release.yml?logo=githubactions)](https://github.com/PRO-2684/osc94/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/osc94?logo=githubactions)](https://github.com/PRO-2684/osc94/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/osc94/total?logo=github)](https://github.com/PRO-2684/osc94/releases)
[![Crates.io Version](https://img.shields.io/crates/v/osc94?logo=rust)](https://crates.io/crates/osc94)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/osc94?logo=rust)](https://crates.io/crates/osc94)
[![docs.rs](https://img.shields.io/docsrs/osc94?logo=rust)](https://docs.rs/osc94)

Library for handling progress bar sequences (OSC 9;4).

## ⚙️ Automatic Releases Setup

1. [Create a new GitHub repository](https://github.com/new) with the name `osc94` and push this generated project to it.
2. Enable Actions for the repository, and grant "Read and write permissions" to the workflow [here](https://github.com/PRO-2684/osc94/settings/actions).
3. [Generate an API token on crates.io](https://crates.io/settings/tokens/new), with the following setup:

    - `Name`: `osc94`
    - `Expiration`: `No expiration`
    - `Scopes`: `publish-new`, `publish-update`
    - `Crates`: `osc94`

4. [Add a repository secret](https://github.com/PRO-2684/osc94/settings/secrets/actions) named `CARGO_TOKEN` with the generated token as its value.
5. Consider removing this section and updating this README with your own project information.

## 📥 Installation

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

## 💡 Examples

TODO

## 📖 Usage

TODO

## 🎉 Credits

TODO
