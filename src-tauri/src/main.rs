// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::Command;
use std::path::Path;
use std::fs;

fn main() {
  tauri::Builder::default()
    // This is where you pass in your commands
    .invoke_handler(tauri::generate_handler![check_file_exists, show_in_folder])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}

#[tauri::command]
fn check_file_exists(file_path: String) -> bool {
    let path = Path::new(&file_path);
    let directory = path.parent().unwrap_or(Path::new(""));
    let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or("");

    let mut found = false;
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with(file_name) {
                        found = true;
                        break;
                    }
                }
            }
        }
    }
    return found.into();
}

#[tauri::command]
fn show_in_folder(path: String) {
  Command::new("explorer")
    .args(["/select,", &path]) // The comma after select is not a typo
    .spawn()
    .unwrap();
}