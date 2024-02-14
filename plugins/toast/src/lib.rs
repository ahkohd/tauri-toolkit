use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, Window,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(target_os = "macos")]
mod macos;

mod structs;
mod toaster;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ToastError {
    #[error("Not implemented")]
    NotImplemented,
    #[error("Failed to create toast window")]
    FailedToCreateToastWindow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToastPosition {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToastOptions {
    pub padding_x: f64,
    pub padding_y: f64,
    pub duration: f64,
    pub position: ToastPosition,
    pub distance: f64,
    pub font_size: f64,
    pub margin_between_toasts: f64,
    pub enter_animation_duration: f64,
    pub exit_animation_duration: f64,
}

impl Default for ToastOptions {
    fn default() -> Self {
        ToastOptions {
            padding_x: 8.0,
            padding_y: 10.0,
            duration: 2.0,
            position: ToastPosition::Bottom,
            distance: 12.0,
            font_size: 16.0,
            margin_between_toasts: 8.0,
            enter_animation_duration: 0.25,
            exit_animation_duration: 0.4,
        }
    }
}

pub type ToastResult = Result<(), ToastError>;

fn show_toast(message: &str, options: ToastOptions) -> ToastResult {
    #[cfg(target_os = "macos")]
    return macos::toast::toast(message, options);

    #[allow(unreachable_code)]
    Err(ToastError::NotImplemented)
}

pub trait ManagerExt<R: Runtime> {
    fn toast(&self, message: &str, options: ToastOptions) -> ToastResult;
}

pub trait WindowExt<R: Runtime> {
    fn toast(&self, message: &str, options: ToastOptions) -> ToastResult;
}

impl<R: Runtime, T: Manager<R>> ManagerExt<R> for T {
    fn toast(&self, message: &str, options: ToastOptions) -> ToastResult {
        show_toast(message, options)
    }
}

impl<R: Runtime> WindowExt<R> for Window<R> {
    fn toast(&self, message: &str, options: ToastOptions) -> ToastResult {
        show_toast(message, options)
    }
}

#[tauri::command]
fn toast<R: Runtime>(
    app_handle: tauri::AppHandle<R>,
    message: String,
    options: Option<ToastOptions>,
) -> ToastResult {
    app_handle.toast(&message, options.unwrap_or_default())
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("toast")
        .invoke_handler(tauri::generate_handler![toast])
        .setup(|app| {
            app.manage(toaster::Toaster::default());
            Ok(())
        })
        .build()
}
