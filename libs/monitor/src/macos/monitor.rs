use std::ffi::CString;

use cocoa::{
    appkit::CGFloat,
    base::{id, nil},
    foundation::{NSPoint, NSRect},
};
use core_foundation::{
    base::{kCFAllocatorDefault, CFRelease},
    mach_port::CFAllocatorRef,
    string::CFStringRef,
    uuid::CFUUIDRef,
};
use core_graphics::display::{CGDirectDisplayID, CGMainDisplayID};
use objc::{
    class, msg_send,
    runtime::{BOOL, NO, YES},
    sel, sel_impl,
};
use tauri::{PhysicalPosition, PhysicalSize};

use crate::Monitor;

use super::utils::nsstring_to_string;

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    pub fn NSMouseInRect(aPoint: NSPoint, aRect: NSRect, flipped: BOOL) -> BOOL;
}

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn CGDisplayCreateUUIDFromDisplayID(display: CGDirectDisplayID) -> CFUUIDRef;

    fn CFUUIDCreateString(allocator: CFAllocatorRef, uuid: CFUUIDRef) -> CFStringRef;
}

pub fn get_monitor_with_cursor() -> Option<Monitor> {
    objc::rc::autoreleasepool(|| {
        let main_display_id: CGDirectDisplayID = unsafe { CGMainDisplayID() };

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

            let device_description_dict: id = unsafe { msg_send![next_screen, deviceDescription] };

            let nsscreen_number: id = unsafe {
                msg_send![class!(NSString), stringWithCString: CString::new("NSScreenNumber").unwrap()]
            };

            let monitor_id: id =
                unsafe { msg_send![device_description_dict, objectForKey: nsscreen_number] };

            let monitor_id: CGDirectDisplayID = unsafe { msg_send![monitor_id, unsignedIntValue] };

            let uuid: Option<String> = {
                let uuid_ref: CFUUIDRef = unsafe { CGDisplayCreateUUIDFromDisplayID(monitor_id) };

                if uuid_ref.is_null() {
                    None
                } else {
                    let uuid_string_ref: CFStringRef =
                        unsafe { CFUUIDCreateString(kCFAllocatorDefault, uuid_ref) };

                    let uuid = nsstring_to_string(uuid_string_ref as _);

                    unsafe {
                        CFRelease(uuid_string_ref.cast());

                        CFRelease(uuid_ref.cast())
                    };

                    uuid
                }
            };

            return Some(Monitor {
                id: monitor_id,
                uuid,
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
                has_cursor: true,
                is_primary: monitor_id == main_display_id,
            });
        }

        None
    })
}

pub fn get_monitors() -> Vec<Monitor> {
    objc::rc::autoreleasepool(|| {
        let main_display_id: CGDirectDisplayID = unsafe { CGMainDisplayID() };

        let mouse_location: NSPoint = unsafe { msg_send![class!(NSEvent), mouseLocation] };

        let screens: id = unsafe { msg_send![class!(NSScreen), screens] };

        let screens_iter: id = unsafe { msg_send![screens, objectEnumerator] };

        let mut next_screen: id;

        let mut monitors = vec![];

        loop {
            next_screen = unsafe { msg_send![screens_iter, nextObject] };

            if next_screen == nil {
                break;
            }

            let frame: NSRect = unsafe { msg_send![next_screen, frame] };

            let is_mouse_in_screen_frame: BOOL =
                unsafe { NSMouseInRect(mouse_location, frame, NO) };

            let screen_name = nsstring_to_string(unsafe { msg_send![next_screen, localizedName] });

            let scale_factor: CGFloat = unsafe { msg_send![next_screen, backingScaleFactor] };

            let scale_factor: f64 = scale_factor;

            let device_description_dict: id = unsafe { msg_send![next_screen, deviceDescription] };

            let nsscreen_number: id = unsafe {
                msg_send![class!(NSString), stringWithCString: CString::new("NSScreenNumber").unwrap()]
            };

            let monitor_id: id =
                unsafe { msg_send![device_description_dict, objectForKey: nsscreen_number] };

            let monitor_id: CGDirectDisplayID = unsafe { msg_send![monitor_id, unsignedIntValue] };

            let uuid: Option<String> = {
                let uuid_ref: CFUUIDRef = unsafe { CGDisplayCreateUUIDFromDisplayID(monitor_id) };

                if uuid_ref.is_null() {
                    None
                } else {
                    let uuid_string_ref: CFStringRef =
                        unsafe { CFUUIDCreateString(kCFAllocatorDefault, uuid_ref) };

                    let uuid = nsstring_to_string(uuid_string_ref as _);

                    unsafe {
                        CFRelease(uuid_string_ref.cast());

                        CFRelease(uuid_ref.cast())
                    };

                    uuid
                }
            };

            monitors.push(Monitor {
                id: monitor_id,
                uuid,
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
                has_cursor: is_mouse_in_screen_frame == YES,
                is_primary: monitor_id == main_display_id,
            });
        }

        monitors
    })
}
