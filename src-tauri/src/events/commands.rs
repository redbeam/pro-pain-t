use std::fs;
use image::RgbaImage;
use pro_pain_t_shared::dtos::image::ImageDto;

/*
!!!
    DON'T FORGET TO ADD THE COMMANDS TO `invoke_handler` IN MAIN.RS
!!!
*/

#[tauri::command(rename_all = "snake_case")]
pub fn save_project_command(path: String, project_serialized: String) {
    fs::write(path, project_serialized).expect("Failed to write to file");
}

#[tauri::command(rename_all = "snake_case")]
pub fn export_project_command(path: String, image: ImageDto) {
    let image = RgbaImage::from_raw(image.width, image.height, image.raw_data)
        .expect("Unable to create image buffer");
    image.save(path).expect("Failed to save image");
}
