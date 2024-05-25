A popover view to `WebviewWindow`.

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
popover = { git = "https://github.com/ahkohd/tauri-toolkit", branch = "v2" }
```

### Demo
See an [example project](https://github.com/ahkohd/tauri-macos-menubar-app-example/tree/popover) that uses this `popover` lib.

<img width="515" alt="image" src="https://github.com/ahkohd/tauri-macos-menubar-app-example/assets/13041443/68cf6e28-5dff-45c1-8fc6-78386839234d">

## Usage
```rust
use popover;

fn main() {
    let window = app_handle.get_webview_window("window_name");

    popover::add_view(&window, None);
}
```

## Functions

- `add_view(window: &tauri::WebviewWindow, options: PopoverConfig)`:
  Adds a popover view to the `WebviewWindow`. If options is `None`, the default options are used.


## PopoverConfig Struct
Here is the description of the fields in this struct:

- `arrow_height`: CGFloat representing the height of popover arrow.
- `arrow_position`: CGFloat representing the horizontal position of arrow.
- `arrow_width`: CGFloat describing the width of the arrow.
- `background_color`: An instance of NSColor determining the background color of the popover.
- `border_color`: NSColor instance representing the border color of the popover.
- `border_width`: CGFloat representing the width of the popover's border.
- `content_edge_insets`: NSEdgeInsets defining the content edge insets of the popover. This typically influences the padding around the content inside the popover.
- `corner_radius`: CGFloat representing the radius of the popover's corners.
- `popover_to_status_item_margin`: CGFloat indicating the margin or distance between the popover and the status item.
- `right_edge_margin`: CGFloat representing the margin or spacing to the right edge of the popover.

To create a new `PopoverConfig`, you can use the following example:

```rust
use obj2_app_kit::{ NSEdgeInsetsZero };

use objc::{ msg_send, class, sel, sel_impl };

use cocoa::base::id;

let background_color: id = unsafe { msg_send![class!(NSColor), windowBackgroundColor] };

let border_color: id = unsafe { msg_send![class!(NSColor), whiteColor] };

let config = PopoverConfig {
    arrow_height: 10.0,
    arrow_position: 100.0,
    arrow_width: 20.0,
    background_color,
    border_color,
    border_width: 2.0,
    content_edge_insets: unsafe { NSEdgeInsetsZero },
    corner_radius: 10.0,
    popover_to_status_item_margin: 10.0,
    right_edge_margin: 12.0,
};
```

This will create a popover with specified configurations in the above example.


## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## License
MIT or MIT/Apache 2.0 where applicable
