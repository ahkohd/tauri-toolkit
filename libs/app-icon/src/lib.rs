use std::path::Path;
use thiserror::Error;

#[cfg(target_os = "macos")]
mod macos;

#[derive(Error, Debug)]
#[error("get app icon error")]
pub struct GetAppIconError {
    #[cfg(target_os = "macos")]
    #[from]
    source: macos::request::GetIconError,
}

#[cfg(target_os = "windows")]
pub fn get_icon(app_path: &Path, save_path: &Path, size: f64) -> Result<(), GetAppIconError> {
    unimplemented!();
}

#[cfg(target_os = "linux")]
pub fn get_icon(app_path: &Path, save_path: &Path, size: f64) -> Result<(), GetAppIconError> {
    unimplemented!();
}

/// Get app icon from app bundle. You specify the path to save the icon, and the desired icon size (like 16, 32, 48, 128, 256, 512)
/// Saves the icon in PNG format.
#[cfg(target_os = "macos")]
pub fn get_icon(app_path: &Path, save_path: &Path, size: f64) -> Result<(), GetAppIconError> {
    macos::request::get_icon(app_path, save_path, size)?;
    Ok(())
}
