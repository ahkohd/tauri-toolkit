#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use crate::macos::popover::PopoverConfig;

#[cfg(target_os = "windows")]
pub fn add_view(window: &tauri::WebviewWindow) {
    unimplemented!();
}

#[cfg(target_os = "linux")]
pub fn add_view(window: &tauri::WebviewWindow) {
    unimplemented!();
}

#[cfg(target_os = "macos")]
pub fn add_view(window: &tauri::WebviewWindow, options: Option<PopoverConfig>) {
    use cocoa::{
        base::id,
        foundation::{NSInteger, NSPoint, NSRect, NSSize},
    };
    use objc::{msg_send, sel, sel_impl};
    use tauri::Manager;

    use crate::macos::popover::PopoverView;

    #[allow(non_upper_case_globals)]
    const NSWindowAnimationBehaviorUtilityWindow: NSInteger = 4;

    let win = window.clone();

    window
        .app_handle()
        .run_on_main_thread(move || {
            let handle: id = win.ns_window().unwrap() as _;

            let content_frame: NSRect = unsafe { msg_send![handle, frame] };

            let content_view: id = unsafe { msg_send![handle, contentView] };

            let mut config = options.unwrap_or_default();

            if options.is_none() {
                config.arrow_position = content_frame.size.width / 2.0;
            }

            let view = PopoverView::new(config);

            let _frame = NSRect::new(
                NSPoint::new(0.0, 0.0),
                NSSize::new(content_frame.size.width, content_frame.size.height),
            );

            view.set_frame(_frame);

            view.set_parent(content_view);

            view.set_autoresizing();

            let () = unsafe {
                msg_send![handle, setAnimationBehavior: NSWindowAnimationBehaviorUtilityWindow]
            };
        })
        .unwrap();
}
