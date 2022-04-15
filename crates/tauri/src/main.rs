#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use backend::app;
use once_cell::sync::OnceCell;

static PORT: OnceCell<u16> = OnceCell::new();

fn main() {
    let port = portpicker::pick_unused_port().expect("failed to find unused port");
    PORT.set(port).unwrap();
    tauri::async_runtime::spawn(app(port));
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_port])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// A command to get the usused port, instead of 3000.
#[tauri::command]
fn get_port() -> Result<String, String> {
    if let Some(port) = PORT.get() {
        println!("{}", port);
        Ok(format!("{}", port))
    } else {
        println!("failed to get port");
        Err("failed to get port".to_string())
    }
}
