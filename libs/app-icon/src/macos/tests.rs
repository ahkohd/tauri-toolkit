#![cfg(test)]
use super::request::{get_icon, GetIconError};
use std::path::Path;

#[test]
fn app_path_does_not_exist() {
    let app_path = Path::new("/foo/bar");
    let save_path = Path::new("/tmp");
    assert_eq!(
        get_icon(app_path, save_path, 32.0).unwrap_err(),
        GetIconError::AppPathDoesNotExist
    );
}

#[test]
fn app_path_without_app_extension() {
    let app_path = Path::new("/System/Applications");
    let save_path = Path::new("/tmp");
    assert_eq!(
        get_icon(app_path, save_path, 32.0).unwrap_err(),
        GetIconError::AppPathDoesNotEndWithApp
    );
}

#[test]
fn save_path_parent_does_not_exist() {
    let app_path = Path::new("/System/Applications/Notes.app");
    let save_path = Path::new("/foo/Notes.png");
    assert_eq!(
        get_icon(app_path, save_path, 32.0).unwrap_err(),
        GetIconError::SavePathParentDirDoesNotExist
    );
}

#[test]
fn it_works() {
    let app_path = Path::new("/System/Applications/Notes.app");
    let save_path = Path::new("/tmp/Notes.png");
    assert!(get_icon(app_path, save_path, 32.0).is_ok());
}
