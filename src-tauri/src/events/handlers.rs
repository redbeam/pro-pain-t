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

pub fn error_dialog(app_handle: &AppHandle, message: impl ToString) {
    app_handle.dialog()
        .message(message.to_string())
        .title("Error")
        .buttons(MessageDialogButtons::Ok)
        .blocking_show();
}

pub fn open_file_handler(app_handle: &AppHandle, file_path: Option<FilePath>) {
    if let Some(path) = file_path {
        let project_file_data = fs::read(path.to_string());
        if project_file_data.is_err() {
            error_dialog(app_handle, "Failed to read file");
            return;
        }
        let project_dto = ProjectDto::new(project_file_data.unwrap());
        if app_handle.emit(EVENT_MENU_OPEN_PROJECT, project_dto).is_err() {
            error_dialog(app_handle, "Failed to emit menubar action");
            return;
        }
        println!("emitted open_project");
    }
}

pub fn save_project_handler(app_handle: &AppHandle, file_path: Option<FilePath>) {
    if let Some(path) = file_path {
        if app_handle.emit(EVENT_MENU_SAVE_PROJECT, PathDto::new(path)).is_err() {
            error_dialog(app_handle, "Failed to emit menubar action");
            return;
        }
        println!("emitted save_project");
    }
}

pub fn import_as_layer_handler(app_handle: &AppHandle, file_path: Option<FilePath>) {
    if let Some(path) = file_path {
        let image = ImageReader::open(path.to_string());
        if image.is_err() {
            error_dialog(app_handle, "Failed to read image file");
            return;
        }
        let image = image.unwrap().decode();
        if image.is_err() {
            error_dialog(app_handle, "Failed to decode image file");
            return;
        }
        let image = image.unwrap().into_rgb8();
        let payload = ImageDto::from_image(image);
        if app_handle.emit(EVENT_MENU_IMPORT_AS_LAYER, payload).is_err() {
            error_dialog(app_handle, "Failed to emit menubar action");
            return;
        }
        println!("emitted import_as_layer");
    }
}

pub fn export_project_handler(app_handle: &AppHandle, file_path: Option<FilePath>) {
    if let Some(path) = file_path {
        if app_handle.emit(EVENT_MENU_EXPORT_PROJECT, PathDto::new(path)).is_err() {
            error_dialog(app_handle, "Failed to emit menubar action");
            return;
        }
        println!("emitted export_project");
    }
}
