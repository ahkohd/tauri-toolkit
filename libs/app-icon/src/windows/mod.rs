use std::fs::File;
use std::io::Write;
use std::os::windows::prelude::*;
use std::path::Path;
use std::ptr::null_mut;

use thiserror::Error;

use windows_sys::Win32::Graphics::Gdi::{
    CreateDIBSection, GetObjectW, BITMAP, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, HDC,
    RGBQUAD,
};
use windows_sys::Win32::UI::Shell::ExtractIconExW;
use windows_sys::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, ICONINFO};

mod tests;

#[derive(Error, Debug, PartialEq)]
pub enum GetIconError {
    #[error("app path does not exist")]
    AppPathDoesNotExist,
    #[error("save path parent directory does not exist")]
    SavePathParentDirDoesNotExist,
    #[error("failed to extract icon")]
    IconExtractionError,
    #[error("failed to get icon info")]
    IconInfoError,
    #[error("failed to convert icon info")]
    IconInfoConversionError,
    #[error("failed to save image")]
    ImageSaveError,
}

pub fn get_icon(app_path: &Path, save_path: &Path, _icon_size: f64) -> Result<(), GetIconError> {
    if !app_path.exists() {
        return Err(GetIconError::AppPathDoesNotExist);
    }

    let parent = save_path
        .parent()
        .ok_or(GetIconError::SavePathParentDirDoesNotExist)?;

    if !parent.exists() {
        return Err(GetIconError::SavePathParentDirDoesNotExist);
    }

    let path: Vec<u16> = app_path.as_os_str().encode_wide().chain(Some(0)).collect();

    let mut large_icon: isize = 0;
    let mut small_icon: isize = 0;

    unsafe {
        let count = ExtractIconExW(path.as_ptr(), 0, &mut large_icon, &mut small_icon, 1);
        if count == 0 {
            return Err(GetIconError::IconExtractionError);
        }

        let mut icon_info: ICONINFO = std::mem::zeroed();
        if GetIconInfo(large_icon, &mut icon_info) == 0 {
            DestroyIcon(large_icon);
            return Err(GetIconError::IconInfoError);
        }

        let mut bitmap: BITMAP = std::mem::zeroed();
        if GetObjectW(
            icon_info.hbmColor as _,
            std::mem::size_of::<BITMAP>() as i32,
            &mut bitmap as *mut _ as *mut _,
        ) == 0
        {
            DestroyIcon(large_icon);
            return Err(GetIconError::IconInfoConversionError);
        }

        // Create a DIB section
        let hdc: HDC = 0;
        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: bitmap.bmWidth,
                biHeight: -bitmap.bmHeight, // Negative to indicate a top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [RGBQUAD {
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0,
            }; 1],
        };

        let pixels: Vec<u8> = vec![0; (bitmap.bmWidth * bitmap.bmHeight * 4) as usize];
        let hbm = CreateDIBSection(
            hdc,
            &bmi as *const _ as *const _,
            DIB_RGB_COLORS,
            &mut null_mut(),
            0,
            0,
        );

        if hbm == 0
            || GetObjectW(
                hbm as _,
                std::mem::size_of::<BITMAP>() as i32,
                &mut bitmap as *mut _ as *mut _,
            ) == 0
        {
            DestroyIcon(large_icon);
            return Err(GetIconError::IconInfoConversionError);
        }

        // Save the bitmap to file
        let file = File::create(save_path).map_err(|_| GetIconError::ImageSaveError)?;
        let mut writer = std::io::BufWriter::new(file);

        // BITMAPFILEHEADER
        writer
            .write_all(&[
                0x42,
                0x4D, // 'BM'
                (bitmap.bmWidth * bitmap.bmHeight * 4 + 54) as u8,
                ((bitmap.bmWidth * bitmap.bmHeight * 4 + 54) >> 8) as u8,
                ((bitmap.bmWidth * bitmap.bmHeight * 4 + 54) >> 16) as u8,
                ((bitmap.bmWidth * bitmap.bmHeight * 4 + 54) >> 24) as u8,
                0,
                0,
                0,
                0,
                54,
                0,
                0,
                0,
            ])
            .map_err(|_| GetIconError::ImageSaveError)?;

        // BITMAPINFOHEADER
        writer
            .write_all(&[
                40,
                0,
                0,
                0,
                (bitmap.bmWidth) as u8,
                ((bitmap.bmWidth) >> 8) as u8,
                ((bitmap.bmWidth) >> 16) as u8,
                ((bitmap.bmWidth) >> 24) as u8,
                (bitmap.bmHeight) as u8,
                ((bitmap.bmHeight) >> 8) as u8,
                ((bitmap.bmHeight) >> 16) as u8,
                ((bitmap.bmHeight) >> 24) as u8,
                1,
                0,
                32,
                0,
                0,
                0,
                0,
                0,
                (bitmap.bmWidth * bitmap.bmHeight * 4) as u8,
                ((bitmap.bmWidth * bitmap.bmHeight * 4) >> 8) as u8,
                ((bitmap.bmWidth * bitmap.bmHeight * 4) >> 16) as u8,
                ((bitmap.bmWidth * bitmap.bmHeight * 4) >> 24) as u8,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ])
            .map_err(|_| GetIconError::ImageSaveError)?;

        // Write the pixels
        writer
            .write_all(&pixels)
            .map_err(|_| GetIconError::ImageSaveError)?;

        // Clean up
        DestroyIcon(large_icon);
    }

    Ok(())
}
