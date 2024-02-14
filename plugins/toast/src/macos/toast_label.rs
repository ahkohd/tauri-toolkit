use cocoa::{
    appkit::NSTextField,
    base::{id, nil},
    foundation::{NSPoint, NSRect, NSSize, NSString},
};
use core_graphics::geometry::CGSize;
use objc::{class, msg_send, runtime::NO, sel, sel_impl};

#[allow(non_upper_case_globals)]
const NSUTF8StringEncoding: i32 = 4;

pub struct ToastLabel(id);

impl ToastLabel {
    fn format(message: &str, font_size: f32) -> String {
        format!(
          "<html><head><meta charset=\"utf-8\"/><style>body {{ font: caption; font-size: {}px; text-align: center}} p {{ display: inline-block }}</style></head><body>{}</body></html>",
          font_size, markdown::to_html(message)
        )
    }

    fn attributed_text(text: &str) -> id {
        let html_text: id = unsafe { NSString::alloc(nil).init_str(text) };
        let html_text: id =
            unsafe { msg_send![html_text, dataUsingEncoding: NSUTF8StringEncoding] };
        let attr_text: id = unsafe { msg_send![class!(NSAttributedString), alloc] };
        let () = unsafe { msg_send![attr_text, initWithHTML: html_text documentAttributes: nil] };
        attr_text
    }

    pub fn get_dimensions(&self) -> CGSize {
        let text_size: CGSize = unsafe { msg_send![self.0, size] };
        text_size
    }

    pub fn new(message: &str, font_size: f32) -> Self {
        let fmt_string = Self::format(message, font_size);
        Self(Self::attributed_text(&fmt_string))
    }

    pub fn make_text_field(&self, padding: (f64, f64)) -> id {
        let label_size = self.get_dimensions();
        let label_rect = NSRect {
            origin: NSPoint {
                x: padding.0,
                y: padding.1,
            },
            size: NSSize {
                width: label_size.width + padding.0 * 2.0,
                height: label_size.height,
            },
        };
        let text_field = unsafe { NSTextField::alloc(nil).initWithFrame_(label_rect) };

        unsafe {
            text_field.setEditable_(NO);
            text_field.setStringValue_(self.0);
        }

        let foreground_color: id = unsafe { msg_send![class!(NSColor), textColor] };
        let () = unsafe { msg_send![text_field, setTextColor: foreground_color] };

        let background_color: id = unsafe { msg_send![class!(NSColor), clearColor] };
        let () = unsafe { msg_send![text_field, setBackgroundColor: background_color] };

        let () = unsafe { msg_send![text_field, setDrawsBackground: NO] };
        let () = unsafe { msg_send![text_field, setBordered: NO] };

        #[allow(non_upper_case_globals)]
        const NSTextAlignmentCenter: i32 = 1;
        let _: () = unsafe { msg_send![text_field, setAlignment: NSTextAlignmentCenter] };

        text_field
    }
}
