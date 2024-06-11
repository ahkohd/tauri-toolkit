use cocoa::{appkit::CGFloat, base::id};
use objc::{class, msg_send, sel, sel_impl};

pub fn get_height() -> CGFloat {
    let status_bar: id = unsafe { msg_send![class!(NSStatusBar), systemStatusBar] };

    let menubar_height: CGFloat = unsafe { msg_send![status_bar, thickness] };

    menubar_height
}
