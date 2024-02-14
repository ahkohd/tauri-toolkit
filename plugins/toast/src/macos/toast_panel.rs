use cocoa::{
    appkit::NSWindowStyleMask,
    base::id,
    foundation::{NSPoint, NSSize},
};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{self, Class, Object, Sel, NO, YES},
    sel, sel_impl, Message,
};
use objc_foundation::INSObject;
use objc_id::Id;

#[allow(dead_code)]
#[repr(C)]
enum CGWindowLevelKey {
    PopUpMenuWindowLevelKey = 11,
}

pub struct ToastPanel;

const CLS_NAME: &str = "ToastPanel";

unsafe impl Send for ToastPanel {}
unsafe impl Sync for ToastPanel {}

unsafe impl Message for ToastPanel {}

impl INSObject for ToastPanel {
    fn class() -> &'static runtime::Class {
        Class::get(CLS_NAME).unwrap_or_else(Self::define_class)
    }
}

impl ToastPanel {
    extern "C" fn dealloc(this: &mut Object, _cmd: Sel) {
        unsafe {
            let superclass = class!(NSObject);
            let dealloc: extern "C" fn(&mut Object, Sel) =
                msg_send![super(this, superclass), dealloc];
            dealloc(this, _cmd);
        }
    }

    fn define_class() -> &'static Class {
        let mut cls = ClassDecl::new(CLS_NAME, class!(NSPanel))
            .unwrap_or_else(|| panic!("Unable to register {} class", CLS_NAME));

        unsafe {
            cls.add_method(
                sel!(dealloc),
                Self::dealloc as extern "C" fn(&mut Object, Sel),
            );
        }

        cls.register()
    }

    pub fn show(&self) {
        let () = unsafe { msg_send![self, orderFrontRegardless] };
    }

    pub fn hide(&self) {
        let () = unsafe { msg_send![self, orderOut] };
    }

    pub fn close(&self) {
        let () = unsafe { msg_send![self, setReleasedWhenClosed: YES] };
        let () = unsafe { msg_send![self, close] };
    }

    pub fn is_visible(&self) -> bool {
        unsafe { msg_send![self, isVisible] }
    }

    pub fn set_size(&self, size: NSSize) -> &Self {
        let () = unsafe { msg_send![self, setContentSize: size] };
        self
    }

    pub fn set_position(&self, position: NSPoint) -> &Self {
        let () = unsafe { msg_send![self, setFrameTopLeftPoint: position] };
        self
    }

    pub fn set_text_field(&self, text_field: id) -> &Self {
        let content_view: id = unsafe { msg_send![self, contentView] };
        let () = unsafe { msg_send![content_view, setWantsLayer: YES] };

        let layer: id = unsafe { msg_send![content_view, layer] };
        let () = unsafe { msg_send![content_view, addSubview: text_field] };

        let bg_color: id = {
            let color: id = unsafe { msg_send![class!(NSColor), textColor] };
            let color_space: id = unsafe { msg_send![class!(NSColorSpace), deviceRGBColorSpace] };
            let color: id = unsafe { msg_send![color, colorUsingColorSpace: color_space] };
            let red_component: f64 = unsafe { msg_send![color, redComponent] };
            let value = if red_component > 0.5 {
                1.0 - red_component
            } else {
                1.0 + red_component
            };
            unsafe {
                msg_send![class!(NSColor), colorWithSRGBRed: value green: value blue: value alpha: 0.8]
            }
        };
        let () = unsafe { msg_send![layer, setBackgroundColor: bg_color] };
        self
    }

    pub fn new() -> Id<Self> {
        let panel: id = unsafe { msg_send![Self::class(), new] };
        let () = unsafe {
            msg_send![panel, setStyleMask: NSWindowStyleMask::NSFullSizeContentViewWindowMask | NSWindowStyleMask::NSBorderlessWindowMask]
        };

        let () = unsafe { msg_send![panel, setOpaque: NO] };
        let clear_color: id = unsafe { msg_send![class!(NSColor), clearColor] };
        let () = unsafe { msg_send![panel, setBackgroundColor: clear_color] };

        let () = unsafe { msg_send![panel, setHasShadow: NO] };
        let () = unsafe { msg_send![panel, setLevel: CGWindowLevelKey::PopUpMenuWindowLevelKey] };
        let () = unsafe { msg_send![panel, setFloatingPanel: YES] };
        let () = unsafe { msg_send![panel, setMovableByWindowBackground: NO] };
        let () = unsafe { msg_send![panel, setHidesOnDeactivate: NO] };

        unsafe { Id::from_retained_ptr(panel as *mut ToastPanel) }
    }
}
