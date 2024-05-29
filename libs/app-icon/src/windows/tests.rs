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
fn it_works() {
    let app_path = Path::new(r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe");
    let save_path = Path::new(r"C:\Windows\Temp\edge.png");
    assert!(get_icon(app_path, save_path, 32.0).is_ok());
}
