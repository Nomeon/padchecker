// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::Command;

fn main() {
  tauri::Builder::default()
    // This is where you pass in your commands
    .invoke_handler(tauri::generate_handler![check_file_exists, show_in_folder])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}

#[tauri::command]
fn check_file_exists(file_path: String) -> String {
    let path = std::path::Path::new(&file_path);
    if path.exists() {
      "exists".into()
    } else {
      "does not exist".into()
    }
}

#[tauri::command]
fn show_in_folder(path: String) {
  Command::new("explorer")
    .args(["/select,", &path]) // The comma after select is not a typo
    .spawn()
    .unwrap();
}