use cocoa::{
    appkit::CGFloat,
    base::{id, nil},
    foundation::{NSPoint, NSRect},
};
use objc::{
    class, msg_send,
    runtime::{BOOL, NO, YES},
    sel, sel_impl,
};

use crate::Menubar;

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    pub fn NSMouseInRect(aPoint: NSPoint, aRect: NSRect, flipped: BOOL) -> BOOL;
}

pub fn get_menubar() -> Option<Menubar> {
    objc::rc::autoreleasepool(|| {
        let mouse_location: NSPoint = unsafe { msg_send![class!(NSEvent), mouseLocation] };

        let screens: id = unsafe { msg_send![class!(NSScreen), screens] };

        let screens_iter: id = unsafe { msg_send![screens, objectEnumerator] };

        let mut next_screen: id;

        let result: Option<(f64, f64)> = loop {
            next_screen = unsafe { msg_send![screens_iter, nextObject] };

            if next_screen == nil {
                break None;
            }

            let frame: NSRect = unsafe { msg_send![next_screen, frame] };

            let visible_frame: NSRect = unsafe { msg_send![next_screen, visibleFrame] };

            let is_mouse_in_screen_frame: BOOL =
                unsafe { NSMouseInRect(mouse_location, frame, NO) };

            if is_mouse_in_screen_frame == YES {
                let menubar_height = frame.size.height
                    - visible_frame.size.height
                    - (visible_frame.origin.y - frame.origin.y)
                    - 1.0;

                let scale_factor: CGFloat = unsafe { msg_send![next_screen, backingScaleFactor] };

                break Some((menubar_height, scale_factor));
            }
        };

        if let Some((height, scale_factor)) = result {
            return Some(Menubar {
                height,
                scale_factor,
            });
        }

        None
    })
}
