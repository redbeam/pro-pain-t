#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menus;
mod events;

use crate::menus::setup_menus;
use crate::events::commands::{error_dialog_command, export_project_command, save_project_command};
use std::env;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            setup_menus(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save_project_command, export_project_command, error_dialog_command])
        .run(tauri::generate_context!())
        .expect("Error while running Pro PainT Tauri application");
}
