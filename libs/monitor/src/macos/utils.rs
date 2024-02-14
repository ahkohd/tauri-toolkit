use std::ffi::{c_char, CStr};

use cocoa::base::id;
use objc::{msg_send, sel, sel_impl};

pub fn nsstring_to_string(ns_string: id) -> Option<String> {
    let utf8: id = unsafe { msg_send![ns_string, UTF8String] };

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
