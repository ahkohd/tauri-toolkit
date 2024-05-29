#![cfg(test)]
use std::path::Path;

use super::{get_icon, GetIconError};

#[test]
fn app_path_does_not_exist() {
    let app_path = Path::new(r"C:\foo\bar");
    let save_path = Path::new(r"C:\foo\temp");
    assert_eq!(
        get_icon(app_path, save_path, 32.0).unwrap_err(),
        GetIconError::AppPathDoesNotExist
    );
}

#[test]
fn save_path_parent_does_not_exist() {
    let app_path = Path::new(r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe");
    let save_path = Path::new(r"Windows\Temp\edge.png");
    assert_eq!(
        get_icon(app_path, save_path, 32.0).unwrap_err(),
        GetIconError::SavePathParentDirDoesNotExist
    );
}

#[test]
fn image_save_failure() {
    let app_path = Path::new(r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe");
    // eleveted access required to write to this folder
    let save_path = Path::new(r"C:\Windows\System32\forbidden_icon.png"); 
    let result = get_icon(app_path, save_path, 32.0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), GetIconError::ImageSaveError);
}

#[test]
fn it_works() {
    let app_path = Path::new(r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe");
    let save_path = Path::new(r"C:\Windows\Temp\edge.png");
    assert!(get_icon(app_path, save_path, 32.0).is_ok());
}
