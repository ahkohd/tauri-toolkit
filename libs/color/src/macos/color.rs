use std::ops::Div;

use cocoa::{
    appkit::{CGFloat, NSColor},
    base::{id, nil},
};
use tauri::window::Color;

pub trait ColorExt {
    fn to_ns_color(&self) -> id;

    fn normalize<T>(x: impl Into<u32>) -> T
    where
        T: From<u8> + Div<Output = T> + From<u8>;
}

impl ColorExt for Color {
    fn normalize<T>(x: impl Into<u32>) -> T
    where
        T: From<u8> + Div<Output = T> + From<u8>,
    {
        let clamped = x.into().min(255) as u8;
        T::from(clamped) / T::from(255u8)
    }

    fn to_ns_color(&self) -> id {
        unsafe {
            NSColor::colorWithSRGBRed_green_blue_alpha_(
                nil,
                Color::normalize::<CGFloat>(self.0),
                Color::normalize::<CGFloat>(self.1),
                Color::normalize::<CGFloat>(self.2),
                Color::normalize::<CGFloat>(self.3),
            )
        }
    }
}
