A library to get information about menubar.

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
menubar = { git = "https://github.com/ahkohd/tauri-toolkit", branch = "v2" }
```

## Usage
```rust
use menubar::get_menubar;

fn main() {
    let menubar = get_menubar();
}
```

## Functions

- `get_menubar() -> Menubar`:
  Get info about the system-wide Menubar.


### Menubar
The struct Menubar provides properties are defined as follows:
```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Menubar {
    height: f64,
}
```
It includes the following fields:
- `height`: the height of the menubar

#### Menubar Methods

`Menubar` struct provides the following methods to fetch its attributes:

- `height(&self) -> f64`: This method returns the height.

To use any of these methods, you need to have an instance of a `Menubar`.

For example: 
```rust
let menubar_height = menubar.height(); 
```

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## License
MIT or MIT/Apache 2.0 where applicable
