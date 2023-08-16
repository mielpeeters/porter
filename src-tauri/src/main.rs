// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use tauri::{AppHandle, Window};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn create_site(
    input_file: &str,
    declaration_file: &str,
    output_file: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let response = porter::declare::create_site(
        declaration_file.into(),
        input_file.into(),
        output_file.into(),
        app_handle,
    );

    match response {
        Ok(_) => Ok("The operation is done.".to_string()),
        Err(err) => Err(format!("{err}")),
    }
}

#[tauri::command]
async fn convert_images(
    input_dir: &str,
    output_dir: &str,
    window: Window,
) -> Result<String, String> {
    let input = PathBuf::from(input_dir);
    let response = porter::images::handle_images(&input, output_dir.into(), window);

    match response {
        Ok(_) => Ok("The operation is done.".to_string()),
        Err(err) => Err(format!("{err}")),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_site, convert_images])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
