use image::ImageReader;
use pro_pain_t_shared::dtos::image::ImageDto;
use pro_pain_t_shared::dtos::path::PathDto;
use pro_pain_t_shared::dtos::project::ProjectDto;
use pro_pain_t_shared::events::events::{EVENT_MENU_EXPORT_PROJECT, EVENT_MENU_IMPORT_AS_LAYER, EVENT_MENU_OPEN_PROJECT, EVENT_MENU_SAVE_PROJECT};
use std::fs;
use tauri::{AppHandle, Emitter};
use tauri_plugin_dialog::{DialogExt, FilePath, MessageDialogButtons};

/// Returns `true` if the user pressed the OK button.
pub fn project_overwrite_confirmation(app_handle: &AppHandle) -> bool {
    app_handle.dialog()
        .message("This will overwrite the currently opened project. Do you want to continue?")
        .title("Warning")
        .buttons(MessageDialogButtons::OkCancelCustom("Yes".to_string(), "No".to_string()))
        .blocking_show()
}

pub fn open_file_handler(app_handle: &AppHandle, file_path: Option<FilePath>) {
    if let Some(path) = file_path {
        let project_file_data = ProjectDto::new(fs::read(path.to_string()).expect("Failed to read file"));
        app_handle.emit(EVENT_MENU_OPEN_PROJECT, project_file_data)
            .expect("Failed to emit menu-open-project");
        println!("emitted open_project");
    }
}

pub fn save_project_handler(app_handle: &AppHandle, file_path: Option<FilePath>) {
    if let Some(path) = file_path {
        app_handle.emit(EVENT_MENU_SAVE_PROJECT, PathDto::new(path))
            .expect("Failed to emit menu-save-project");
        println!("emitted save_project");
    }
}

pub fn import_as_layer_handler(app_handle: &AppHandle, file_path: Option<FilePath>) {
    if let Some(path) = file_path {
        let image = ImageReader::open(path.to_string()).expect("Failed to read image file")
            .decode().expect("Failed to decode image file")
            .into_rgb8();
        let payload = ImageDto::from_image(image);
        app_handle.emit(EVENT_MENU_IMPORT_AS_LAYER, payload)
            .expect("Failed to emit menu-import-as-layer");
        println!("emitted import_as_layer");
    }
}

pub fn export_project_handler(app_handle: &AppHandle, file_path: Option<FilePath>) {
    if let Some(path) = file_path {
        app_handle.emit(EVENT_MENU_EXPORT_PROJECT, PathDto::new(path))
            .expect("Failed to emit menu-export-project");
        println!("emitted export_project");
    }
}
