use std::ffi::{c_char, CStr};

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
use serde::{Deserialize, Serialize};
use tauri::{PhysicalPosition, PhysicalSize};

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    pub fn NSMouseInRect(aPoint: NSPoint, aRect: NSRect, flipped: BOOL) -> BOOL;
}

fn nsstring_to_string(value: id) -> Option<String> {
    let utf8: id = unsafe { msg_send![value, UTF8String] };
    if !utf8.is_null() {
        Some(unsafe {
            {
                CStr::from_ptr(utf8 as *const c_char)
                    .to_string_lossy()
                    .into_owned()
            }
        })
    } else {
        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Monitor {
    pub name: Option<String>,
    pub size: PhysicalSize<u32>,
    pub position: PhysicalPosition<i32>,
    pub scale_factor: f64,
}

impl Monitor {
    /// Returns the monitor's resolution.
    pub fn size(&self) -> &PhysicalSize<u32> {
        &self.size
    }

    /// Returns the top-left corner position of the monitor relative to the larger full screen area.
    pub fn position(&self) -> &PhysicalPosition<i32> {
        &self.position
    }

    /// Returns the scale factor that can be used to map logical pixels to physical pixels, and vice versa.
    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }
}

pub fn get_primary_monitor() -> Monitor {
    let screens: id = unsafe { msg_send![class!(NSScreen), screens] };
    let screen: id = unsafe { msg_send![screens, objectAtIndex: 0] };
    let frame: NSRect = unsafe { msg_send![screen, frame] };
    let screen_name = nsstring_to_string(unsafe { msg_send![screen, localizedName] });
    let scale_factor: CGFloat = unsafe { msg_send![screen, backingScaleFactor] };
    let scale_factor: f64 = scale_factor;

    Monitor {
        name: screen_name,
        position: PhysicalPosition {
            x: (frame.origin.x * scale_factor) as i32,
            y: (frame.origin.y * scale_factor) as i32,
        },
        size: PhysicalSize {
            width: (frame.size.width * scale_factor) as u32,
            height: (frame.size.height * scale_factor) as u32,
        },
        scale_factor,
    }
}

pub fn get_monitor_with_cursor() -> Option<Monitor> {
    objc::rc::autoreleasepool(|| {
        let mouse_location: NSPoint = unsafe { msg_send![class!(NSEvent), mouseLocation] };
        let screens: id = unsafe { msg_send![class!(NSScreen), screens] };
        let screens_iter: id = unsafe { msg_send![screens, objectEnumerator] };
        let mut next_screen: id;

        let frame_with_cursor: Option<NSRect> = loop {
            next_screen = unsafe { msg_send![screens_iter, nextObject] };
            if next_screen == nil {
                break None;
            }

            let frame: NSRect = unsafe { msg_send![next_screen, frame] };
            let is_mouse_in_screen_frame: BOOL =
                unsafe { NSMouseInRect(mouse_location, frame, NO) };
            if is_mouse_in_screen_frame == YES {
                break Some(frame);
            }
        };

        if let Some(frame) = frame_with_cursor {
            let screen_name = nsstring_to_string(unsafe { msg_send![next_screen, localizedName] });
            let scale_factor: CGFloat = unsafe { msg_send![next_screen, backingScaleFactor] };
            let scale_factor: f64 = scale_factor;

            return Some(Monitor {
                name: screen_name,
                position: PhysicalPosition {
                    x: (frame.origin.x * scale_factor) as i32,
                    y: (frame.origin.y * scale_factor) as i32,
                },
                size: PhysicalSize {
                    width: (frame.size.width * scale_factor) as u32,
                    height: (frame.size.height * scale_factor) as u32,
                },
                scale_factor,
            });
        }

        None
    })
}
