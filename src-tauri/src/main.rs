#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menus;

use std::time::Duration;
use tauri::Emitter;
use crate::menus::setup_menus;

const AUTOSAVE_INTERVAL: u32 = 180; // seconds

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            setup_menus(app)?;

            let app_handle = app.handle().clone();
            std::thread::spawn(move || loop {
                std::thread::sleep(Duration::from_secs(AUTOSAVE_INTERVAL as u64));
                app_handle.emit("autosave-project", ())
                    .expect("Failed to emit autosave-project");
                println!("emitted autosave-project");
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Pro PainT Tauri application");
}
