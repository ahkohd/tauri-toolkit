Listen to System or Workspace notification.

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
system-notification = { git = "https://github.com/ahkohd/tauri-toolkit", branch = "v1" }
```

### Usage

```rust
use system_notification::WorkspaceListener;

fn main() {
    // ...

    // listen to a workspace notification
    app_handle.listen_workspace("NSWorkspaceDidActivateApplicationNotification", |app_handle| {
       // An app was activated, do something here...
    });

    // listen to a notification sent by the system to the application
    app_handle.listen_notification("NSSystemColorsDidChangeNotification", |app_handle| {
       // System colors have changed, do something here...
    });

    // ...
}
```

## Functions
The `WorkspaceListener` trait from the `system-notification` crate when in scope adds the following methods to the `AppHandle`.

- `listen_workspace(&str, notification_name: &str, callback: fn(AppHandle<R: Runtime>))`:
  Listen to a workspace notification.

- `listen_notification(&str, notification_name: &str, callback: fn(AppHandle<R: Runtime>))`:
  Listen to a system notification sent to the application's default notification center.

  **Prameters:**
  - `notification_name` _&str_ The notification name. See https://developer.apple.com/documentation/foundation/nsnotificationname for an exhaustive list of notification names.
  - `callback` _fn(AppHandle<R: Runtime>)_ A callback function.

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## License
MIT or MIT/Apache 2.0 where applicable
