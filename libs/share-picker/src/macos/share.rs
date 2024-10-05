use std::path::PathBuf;

use objc2::{msg_send_id, rc::Retained, ClassType};
use objc2_app_kit::{NSSharingServicePicker, NSView, NSWindow};
use objc2_foundation::{
    MainThreadMarker, NSArray, NSPoint, NSRect, NSRectEdge, NSSize, NSString, NSURL,
};
use tauri::{PhysicalPosition, Runtime, WebviewWindow};

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
    fn share(
        &self,
        items: Vec<PathBuf>,
        position: PhysicalPosition<f64>,
        preferred_edge: PreferredEdge,
    );
}

impl<R: Runtime> SharePicker<R> for WebviewWindow<R> {
    fn share(
        &self,
        items: Vec<PathBuf>,
        position: PhysicalPosition<f64>,
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

        let scale_factor = self.scale_factor().unwrap();

        let position = PhysicalPosition {
            x: position.x,
            y: window.frame().size.height - position.y,
        };

        let point_in_window = NSPoint::new(position.x, position.y);

        let point_in_content_view = content_view.convertPoint_fromView(point_in_window, None);

        let mtm = MainThreadMarker::new().unwrap();

        let view = unsafe { NSView::new(mtm) };

        unsafe {
            view.setFrame(NSRect::new(point_in_content_view, NSSize::new(1.0, 1.0)));

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
