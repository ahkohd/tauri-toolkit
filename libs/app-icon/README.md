A library to get the app icon from an app bundle.

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
app-icon = { git = "https://github.com/ahkohd/tauri-toolkit", branch = "main" }
```
## Usage
```rust
use app_icon::GetAppIconError;

fn main() -> Result<(), GetAppIconError> {
    let app_path = Path::new("/System/Applications/Notes.app");
    let save_path = Path::new("/tmp/Notes.png");
    app_icon::get_icon(app_path, save_path, 32.0)?;
    Ok(())
}
```

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## License
MIT or MIT/Apache 2.0 where applicable
