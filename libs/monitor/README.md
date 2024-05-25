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
monitor = { git = "https://github.com/ahkohd/tauri-toolkit", branch = "v2" }
```

## Usage
```rust
use monitor::{get_monitors, get_monitor_with_cursor};

fn main() {
    let monitors = get_monitors();

    let monitor_with_cursor = get_monitor_with_cursor();
}
```

## Functions

- `get_monitor_with_cursor() -> Option<Monitor>`:
  Returns the monitor which currently hosts the system pointer, if any.

- `get_monitors() -> Vec<Monitor>`:
  Returns a vector of all connected monitors.

### Monitor
The struct Monitor provides properties of a single display monitor, defined as follows:
```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Monitor {
    id: u32,
    uuid: Option<String>,
    name: Option<String>,
    size: PhysicalSize<f64>,
    position: PhysicalPosition<f64>,
    scale_factor: f64,
    has_cursor: bool,
    is_primary: bool,
    visible_area: VisibleArea,
}
```
It includes the following fields:
- `id`: a unique identifier for the monitor
- `uuid`: the UUID of the monitor, if any
- `name`: the name of the monitor, if available
- `size`: the size of the monitor, specified as a PhysicalSize struct
- `position`: the position of the monitor, specified as a PhysicalPosition struct
- `scale_factor`: the scaling factor of the monitor's resolution
- `has_cursor`: a Boolean flag indicating if the monitor currently has a cursor
- `is_primary`: a Boolean flag indicating if the monitor is the primary monitor
- `visible_area`: the visible area of the monitor

#### Monitor Methods

`Monitor` struct provides the following methods to fetch its attributes:

- `id(&self) -> u32`: This method returns the identifier of the monitor which is of type `u32`.

- `uuid(&self) -> Option<&String>`: This method returns an `Option` containing a reference to the UUID of the monitor, if it is assigned one.

- `name(&self) -> Option<&String>`: This method returns an `Option` containing a reference to the name of the monitor, if it is assigned one.

- `size(&self) -> PhysicalSize<f64>`: This method returns the size of the monitor as an instance of `PhysicalSize<f64>`.

- `visible_area(&self) -> VisibleArea`: This method returns the visible area of the monitor as a `VisibleArea` struct.

- `position(&self) -> PhysicalPosition<f64>`: This method returns the position of the monitor as an instance of `PhysicalPosition<f64>`.

- `scale_factor(&self) -> f64`: This method returns the scale factor of the monitor.

- `has_cursor(&self) -> bool`: This method returns a boolean value indicating whether the monitor currently has a cursor.

- `is_primary(&self) -> bool`: This method returns a boolean value indicating whether or not the monitor is the primary monitor.

To use any of these methods, you need to have an instance of a `Monitor`.

For example: 
```rust
let monitor_id = monitor.id(); 
let monitor_uuid = monitor.uuid();
```

### VisibleArea

This visible area is represented by the struct `VisibleArea` defined as follows:

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisibleArea {
    size: PhysicalSize<f64>,
    position: PhysicalPosition<f64>,
}
```
It includes fields:
- `size`: the size of the visible area specified as a PhysicalSize struct containing width and height as f64.
- `position`: the position of the visible area on screen specified as a PhysicalPosition struct containing x and y as f64.

#### VisibleArea Methods

`VisibleArea` struct provides the following methods:

- `size(&self) -> PhysicalSize<f64>`: This method returns the size of the visible area as an instance of `PhysicalSize<f64>`.

- `position(&self) -> PhysicalPosition<f64>`: This method returns the position of the visible area on the screen as an instance of `PhysicalPosition<f64>`.

Each of these methods allow you to access specific attributes of the `VisibleArea`.

For example:
```rust
let visible_area_size = visible_area.size();
let visible_area_position = visible_area.position();
```

These methods are particularly useful when you need to query or manipulate the visible area of a screen. Just instantiate a `VisibleArea` and call the appropriate methods.

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## License
MIT or MIT/Apache 2.0 where applicable
