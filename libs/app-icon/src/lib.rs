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

#[cfg(target_os = "macos")]
pub fn get_icon(app_path: &Path, save_path: &Path, size: f64) -> Result<(), GetAppIconError> {
    macos::request::get_icon(app_path, save_path, size)?;
    Ok(())
}
