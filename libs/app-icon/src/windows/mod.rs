use std::num::TryFromIntError;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::{
    mem::{self, MaybeUninit},
    ptr::addr_of_mut,
};

use image::RgbaImage;
use thiserror::Error;
use windows_sys::Win32::Graphics::Gdi::{
    DeleteObject, GetDC, GetDIBits, GetObjectW, ReleaseDC, BITMAP, BITMAPINFOHEADER, BI_RGB,
    DIB_RGB_COLORS,
};
use windows_sys::Win32::System::Com::CoUninitialize;
use windows_sys::Win32::UI::Shell::ExtractIconExW;
use windows_sys::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON};

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
    #[error("Failed to convert one of the bitmap data to valid integer: {0}")]
    BitmapConversionError(#[from] TryFromIntError),
}

unsafe fn icon_to_image(icon: HICON) -> Result<RgbaImage, GetIconError> {
    let mut icon_info = MaybeUninit::uninit();
    if GetIconInfo(icon, icon_info.as_mut_ptr()) == 0 {
        return Err(GetIconError::IconInfoError);
    }
    let icon_info = icon_info.assume_init();

    DeleteObject(icon_info.hbmMask);

    // rough way to get bitmap structure for icon
    let bitmap_size_i32 = i32::try_from(mem::size_of::<BITMAP>())?;
    let mut bitmap: MaybeUninit<BITMAP> = MaybeUninit::uninit();
    let result = GetObjectW(
        icon_info.hbmColor,
        bitmap_size_i32,
        bitmap.as_mut_ptr().cast(),
    );
    if result == 0 {
        DeleteObject(icon_info.hbmColor);
        return Err(GetIconError::IconInfoConversionError);
    }
    let bitmap = bitmap.assume_init();

    let width_u32 = u32::try_from(bitmap.bmWidth)?;
    let height_u32 = u32::try_from(bitmap.bmHeight)?;
    let width_usize = usize::try_from(bitmap.bmWidth)?;
    let height_usize = usize::try_from(bitmap.bmHeight)?;
    let buf_size = width_usize
        .checked_mul(height_usize)
        .and_then(|size| size.checked_mul(4))
        .ok_or(GetIconError::IconInfoConversionError)?;
    let mut buf: Vec<u8> = Vec::with_capacity(buf_size);

    // device context
    let dc = GetDC(0);
    if dc == 0 {
        DeleteObject(icon_info.hbmColor);
        return Err(GetIconError::IconInfoError);
    }

    let biheader_size_u32 = u32::try_from(mem::size_of::<BITMAPINFOHEADER>())?;
    let mut bitmap_info = BITMAPINFOHEADER {
        biSize: biheader_size_u32,
        biWidth: bitmap.bmWidth,
        // i'm using negative sign here to indicate that DIB should from top to bottom (i.e top down)
        biHeight: -bitmap.bmHeight,
        biPlanes: 1,
        biBitCount: 32,
        biCompression: BI_RGB,
        biSizeImage: 0,
        biXPelsPerMeter: 0,
        biYPelsPerMeter: 0,
        biClrUsed: 0,
        biClrImportant: 0,
    };

    let result = GetDIBits(
        dc,
        icon_info.hbmColor,
        0,
        height_u32,
        buf.as_mut_ptr().cast(),
        addr_of_mut!(bitmap_info).cast(),
        DIB_RGB_COLORS,
    );
    if result == 0 {
        DeleteObject(icon_info.hbmColor);
        ReleaseDC(0, dc);
        return Err(GetIconError::IconInfoConversionError);
    }
    buf.set_len(buf.capacity());

    ReleaseDC(0, dc);
    DeleteObject(icon_info.hbmColor);

    // swap the red and blue channels
    for chunk in buf.chunks_exact_mut(4) {
        let [b, _, r, _] = chunk else { unreachable!() };
        mem::swap(b, r);
    }

    RgbaImage::from_vec(width_u32, height_u32, buf).ok_or(GetIconError::ImageSaveError)
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
            CoUninitialize();
            return Err(GetIconError::IconExtractionError);
        }

        let image = icon_to_image(large_icon).map_err(|e| {
            DestroyIcon(large_icon);
            CoUninitialize();
            e
        })?;

        DestroyIcon(large_icon);
        image
            .save(save_path)
            .map_err(|_| GetIconError::ImageSaveError)?;
        CoUninitialize();
    }

    Ok(())
}
