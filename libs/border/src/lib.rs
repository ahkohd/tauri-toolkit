#[cfg(target_os = "macos")]
use cocoa::foundation::NSRect;

#[cfg(target_os = "macos")]
use cocoa::{
    base::id,
    foundation::{NSPoint, NSSize},
};

#[cfg(target_os = "macos")]
use macos::border::{BorderView, BorderViewConfig};

#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl};

use tauri::{Runtime, WebviewWindow};

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "macos")]
pub type BorderConfig = macos::border::BorderViewConfig;

#[cfg(target_os = "macos")]
pub trait WebviewWindowExt {
    fn add_border(&self, config: Option<BorderViewConfig>);
}

#[cfg(target_os = "macos")]
impl<R: Runtime> WebviewWindowExt for WebviewWindow<R> {
    fn add_border(&self, config: Option<BorderViewConfig>) {
        let handle: id = self.ns_window().unwrap() as _;

        let content_frame: NSRect = unsafe { msg_send![handle, frame] };

        let content_view: id = unsafe { msg_send![handle, contentView] };

        let view = BorderView::new(config);

        let frame = NSRect::new(
            NSPoint::new(0.0, 0.0),
            NSSize::new(content_frame.size.width, content_frame.size.height),
        );

        view.set_frame(frame);

        view.set_parent(content_view);

        view.set_autoresizing();
    }
}
