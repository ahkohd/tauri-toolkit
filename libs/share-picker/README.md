Show the Share picker over a `WebviewWindow`.

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
share-picker = { git = "https://github.com/ahkohd/tauri-toolkit", branch = "v2" }
```

### Demo
![A demo of the share picker](https://github.com/ahkohd/tauri-toolkit/blob/v2/assets/share-picker.gif)

## Usage
```rust
use share_picker::{SharePicker, PreferredEdge};
use tauri::PhysicalPosition;

fn main() {
    let window = app_handle.get_webview_window("window_name");

    let item = Path::from("/foo/bar.pdf");

    window.share(vec![item.to_path_buf()], PhysicalPosition {
        x: 0.0,
        y: 0.0
    }, PreferredEdge::BottomLeft);
}
```

## Functions

- `share(window: &tauri::WebviewWindow, items: Vec<PathBuf>, position: PhysicalPosition<f64>, preferred_edge: PreferredEdge)`:
  Displays the Share picker at the cursor position within a WebviewWindow. 
  - `items: Vec<PathBuf>`: A list of paths to items to share.
  - `position: PhysicalPosition<f64>`: Set the position to display the share picker. The origin is the top left corner of the window.
  - `preferred_edge: PreferredEdge`: The preferred edge for displaying the share picker at the cursor position's rectangle.


## PreferredEdge Enum
- `TopLeft`: Place at the top left edge.
- `TopRight`: Place at the top right edge.
- `BottomLeft`: Place at the bottom left edge.
- `BottomRight`: Place at the bottom right edge.

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## License
MIT or MIT/Apache 2.0 where applicable
