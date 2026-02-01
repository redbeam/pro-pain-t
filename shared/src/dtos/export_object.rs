use serde::{Deserialize, Serialize};
use crate::dtos::image::ImageDto;

#[derive(Clone, Serialize, Deserialize)]
pub struct ExportObjectDto {
    pub path: String,
    pub image: ImageDto,
}

impl ExportObjectDto {
    pub fn new(path: String, image: ImageDto) -> Self {
        Self { path, image }
    }
}
