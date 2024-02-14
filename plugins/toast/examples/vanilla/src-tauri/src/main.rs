// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_toast::{ManagerExt, ToastError};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(app_handle: tauri::AppHandle, name: &str) -> Result<(), ToastError> {
    let message = format!("Hello, {}! You've been greeted from Rust!", name);
    app_handle.toast(&message, Default::default())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_toast::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
