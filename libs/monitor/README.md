A library to get information about monitors.

### Install
_This lib requires a Rust version of at least **1.64**_

There are three general methods of installation that we can recommend.

1. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)
2. Pull sources directly from Github using git tags / revision hashes (most secure)
3. Git submodule install this repo in your tauri project and then use file protocol to ingest the source (most secure, but inconvenient to use)

Install the Core plugin by adding the following to your `Cargo.toml` file:

`src-tauri/Cargo.toml`
```toml
[dependencies]
monitor = { git = "https://github.com/ahkohd/tauri-toolkit", branch = "main" }
```
## Usage
```rust
use monitor::{get_monitors, get_monitor_with_cursor};

fn main() {
    // get all monitors
    let monitors = get_monitors();

    // get the monitor with cursor
    let monitor_with_cursor = get_monitor_with_cursor();
}
```

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## License
MIT or MIT/Apache 2.0 where applicable
