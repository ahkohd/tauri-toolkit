use cocoa::foundation::{NSPoint, NSSize};
use core_graphics::display::CGSize;
use objc_id::Id;

use crate::{ToastError, ToastOptions, ToastPosition, ToastResult};

use super::{toast_label::ToastLabel, toast_panel::ToastPanel};

pub struct Toast {
    panel: Id<ToastPanel>,
}

impl Toast {
    fn new(message: &str, options: &ToastOptions) -> Self {
        let label = ToastLabel::new(message, 16.0);
        let label_size = label.get_dimensions();

        let toast_size = Toast::dimensions(options.padding_x, options.padding_y, label_size);
        let toast_position = Toast::position(&toast_size, options);
        let toast = ToastPanel::new();
        toast.set_size(toast_size).set_position(toast_position);

        let text_field = label.make_text_field((options.padding_x, options.padding_y));
        toast.set_text_field(text_field);

        toast.show();
        Self { panel: toast }
    }

    fn dimensions(padding_x: f64, padding_y: f64, text_size: CGSize) -> NSSize {
        let width = text_size.width + padding_x * 4.0;
        let height = text_size.height + padding_y * 2.0;
        NSSize::new(width, height)
    }

    fn position(panel_size: &NSSize, options: &ToastOptions) -> NSPoint {
        let monitor =
            super::utils::get_monitor_with_cursor().unwrap_or(super::utils::get_primary_monitor());
        let scale_factor = monitor.scale_factor;
        let size = monitor.size.to_logical::<f64>(scale_factor);
        let position = monitor.position.to_logical::<f64>(scale_factor);
        let distance_from_edge = (size.height * options.distance / 100.0) * scale_factor;

        NSPoint {
            x: ((size.width / 2.0) - (panel_size.width / 2.0)) + position.x,
            y: match options.position {
                ToastPosition::Top => position.y + distance_from_edge,
                ToastPosition::Bottom => position.y + size.height - distance_from_edge,
            },
        }
    }
}

pub fn toast(message: &str, options: ToastOptions) -> ToastResult {
    let toast = Toast::new(message, &options);
    Err(ToastError::NotImplemented)
}
