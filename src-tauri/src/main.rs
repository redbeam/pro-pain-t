#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menus;

use crate::menus::setup_menus;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            setup_menus(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Pro PainT Tauri application");
}
