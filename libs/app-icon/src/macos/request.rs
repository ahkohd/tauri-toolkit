use std::{ffi::CString, path::Path};

use cocoa::{
    base::{id, nil, NO, YES},
    foundation::{NSInteger, NSPoint, NSRect, NSSize},
};
use objc::{class, msg_send, rc::autoreleasepool, sel, sel_impl};
use thiserror::Error;

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum NSBitmapImageFileType {
    NSBitmapImageFileTypePNG = 4,
}

#[link(name = "AppKit", kind = "framework")]
extern "C" {
    pub static NSDeviceRGBColorSpace: id;
}

#[derive(Error, Debug, PartialEq)]
pub enum GetIconError {
    #[error("app path does not exist")]
    AppPathDoesNotExist,
    #[error("app path does not have '.app' extension")]
    AppPathDoesNotEndWithApp,
    #[error("save path parent directory does not exist")]
    SavePathParentDirDoesNotExist,
    #[error("failed to convert {0} to &str")]
    PathConversionError(&'static str),
    #[error("Failed to create a CString from the path")]
    CStringCreationError(#[from] std::ffi::NulError),
}

pub fn get_icon(app_path: &Path, save_path: &Path, icon_size: f64) -> Result<(), GetIconError> {
    if !app_path.exists() {
        return Err(GetIconError::AppPathDoesNotExist);
    }

    if !app_path.extension().map_or(false, |ext| ext == "app") {
        return Err(GetIconError::AppPathDoesNotEndWithApp);
    }

    let parent = save_path
        .parent()
        .ok_or(GetIconError::SavePathParentDirDoesNotExist)?;

    if !parent.exists() {
        return Err(GetIconError::SavePathParentDirDoesNotExist);
    }

    autoreleasepool(|| unsafe {
        let app_path = app_path
            .to_str()
            .ok_or(GetIconError::PathConversionError("app_path"))
            .map(CString::new)?
            .map_err(GetIconError::CStringCreationError)?;
        let save_path = save_path
            .to_str()
            .ok_or(GetIconError::PathConversionError("save_path"))
            .map(CString::new)?
            .map_err(GetIconError::CStringCreationError)?;
        let nsstring_app_path: id = msg_send![class!(NSString), stringWithCString: app_path];
        let nsstring_save_path: id = msg_send![class!(NSString), stringWithCString: save_path];

        let nsworkspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let nsimage: id = msg_send![nsworkspace, iconForFile: nsstring_app_path];
        let () = msg_send![nsimage, setSize: NSSize::new(icon_size, icon_size)];

        let bits_per_sample: NSInteger = 8;
        let samples_per_pixel: NSInteger = 4;
        let zero: NSInteger = 0;

        let bitmap_ref: id = msg_send![class!(NSBitmapImageRep), alloc];
        let image_rep: id = msg_send![bitmap_ref, initWithBitmapDataPlanes:nil pixelsWide:icon_size as NSInteger pixelsHigh:icon_size as NSInteger bitsPerSample:bits_per_sample samplesPerPixel:samples_per_pixel hasAlpha:YES isPlanar:NO colorSpaceName:NSDeviceRGBColorSpace bytesPerRow:zero bitsPerPixel:zero];
        let () = msg_send![image_rep, setSize: NSSize::new(icon_size, icon_size)];

        let () = msg_send![class!(NSGraphicsContext), saveGraphicsState];
        let context: id = msg_send![
          class!(NSGraphicsContext),
          graphicsContextWithBitmapImageRep: image_rep
        ];
        let () = msg_send![class!(NSGraphicsContext), setCurrentContext: context];
        let () = msg_send![nsimage, drawInRect: NSRect {
            origin: NSPoint {
              x: 0.0,
              y: 0.0,
            },
            size: NSSize {
              width: icon_size,
              height: icon_size,
            },
          } fromRect:NSRect {
            origin: NSPoint {
              x: 0.0,
              y: 0.0,
            },
            size: NSSize {
              width: 0.0,
              height: 0.0,
            },
          } operation:cocoa::appkit::NSCompositingOperation::NSCompositeCopy fraction:1.0];

        let () = msg_send![class!(NSGraphicsContext), restoreGraphicsState];
        let png_data: id = msg_send![image_rep, representationUsingType:NSBitmapImageFileType::NSBitmapImageFileTypePNG properties:nil];
        let () = msg_send![png_data, writeToFile:nsstring_save_path atomically:YES];
        let () = msg_send![image_rep, autorelease];

        Ok(())
    })
}
