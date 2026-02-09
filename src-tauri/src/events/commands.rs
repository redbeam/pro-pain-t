use std::fs;
use image::RgbaImage;
use tauri::AppHandle;
use pro_pain_t_shared::dtos::image::ImageDto;
use crate::events::handlers::error_dialog;
/*
!!!
    DON'T FORGET TO ADD THE COMMANDS TO `invoke_handler` IN MAIN.RS
!!!
*/

#[tauri::command(rename_all = "snake_case")]
pub fn save_project_command(app_handle: AppHandle, path: String, project_serialized: String) {
    if fs::write(path, project_serialized).is_err() {
        error_dialog(&app_handle, "Failed to write project to file");
        return;
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn export_project_command(app_handle: AppHandle, path: String, image: ImageDto) {
    let image = RgbaImage::from_raw(image.width, image.height, image.raw_data);
    if image.is_none() {
        error_dialog(&app_handle, "Failed to create image buffer");
        return;
    }
    if image.unwrap().save(path).is_err() {
        error_dialog(&app_handle, "Failed to save image");
        return;
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn error_dialog_command(app_handle: AppHandle, message: String) {
    error_dialog(&app_handle, message);
}
