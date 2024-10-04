use std::path::PathBuf;

use objc2::{msg_send_id, rc::Retained, ClassType};
use objc2_app_kit::{NSEvent, NSSharingServicePicker, NSView, NSWindow};
use objc2_foundation::{
    MainThreadMarker, NSArray, NSPoint, NSRect, NSRectEdge, NSSize, NSString, NSURL,
};
use tauri::{Runtime, WebviewWindow};

pub enum PreferredEdge {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl PreferredEdge {
    fn to_ns_rect_edge(&self) -> NSRectEdge {
        match self {
            Self::TopLeft => NSRectEdge::NSMinXEdge,
            Self::TopRight => NSRectEdge::NSMaxXEdge,
            Self::BottomLeft => NSRectEdge::NSMinYEdge,
            Self::BottomRight => NSRectEdge::NSMaxYEdge,
        }
    }
}

pub trait SharePicker<R: Runtime> {
    fn share(&self, items: Vec<PathBuf>, offset: Option<(f64, f64)>, preferred_edge: PreferredEdge);
}

impl<R: Runtime> SharePicker<R> for WebviewWindow<R> {
    fn share(
        &self,
        items: Vec<PathBuf>,
        offset: Option<(f64, f64)>,
        preferred_edge: PreferredEdge,
    ) {
        let items = items
            .iter()
            .map(|url| NSString::from_str(url.to_str().unwrap()))
            .collect::<Vec<Retained<NSString>>>();

        let urls = items
            .iter()
            .map(|url| unsafe { NSURL::fileURLWithPath(url) })
            .collect::<Vec<Retained<NSURL>>>();

        let picker: Retained<NSSharingServicePicker> = unsafe {
            let items = NSArray::from_vec(urls);

            msg_send_id![NSSharingServicePicker::alloc(), initWithItems: items.as_ref()]
        };

        let window = self.ns_window().unwrap();

        let window = unsafe { (window.cast() as *mut NSWindow).as_ref().unwrap() };

        let content_view = window.contentView().unwrap();

        let mouse_location_in_screen = unsafe { NSEvent::mouseLocation() };

        let mouse_location_in_window = window.convertPointFromScreen(mouse_location_in_screen);

        let mouse_location_in_content_view =
            content_view.convertPoint_fromView(mouse_location_in_window, None);

        let mtm = MainThreadMarker::new().unwrap();

        let view = unsafe { NSView::new(mtm) };

        unsafe {
            view.setFrame(NSRect::new(
                if let Some((x, y)) = offset {
                    NSPoint::new(
                        mouse_location_in_content_view.x + x,
                        mouse_location_in_content_view.y + y,
                    )
                } else {
                    mouse_location_in_content_view
                },
                NSSize::new(1.0, 1.0),
            ));

            content_view.addSubview(&view);
        }

        unsafe {
            picker.standardShareMenuItem(mtm);

            picker.showRelativeToRect_ofView_preferredEdge(
                NSRect::ZERO,
                view.as_ref(),
                preferred_edge.to_ns_rect_edge(),
            )
        }
    }
}
