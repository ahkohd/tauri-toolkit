use std::path::Path;

use super::{get_icon, GetIconError};

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
fn it_works() {
    let app_path = Path::new(r"C:\Program Files\Git\git-bash.exe");
    let save_path = Path::new(r"C:\Windows\Temp\fit.png");
    match get_icon(app_path, save_path, 64.0) {
        Ok(info) => println!("Icon Information"),
        Err(e) => eprintln!("Error: {}", e),
    }}