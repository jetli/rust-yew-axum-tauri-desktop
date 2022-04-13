#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use backend::app;

fn main() {
    tauri::async_runtime::spawn(app());
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
