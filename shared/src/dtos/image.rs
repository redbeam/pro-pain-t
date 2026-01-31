use image::RgbImage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ImageDto {
    pub width: u32,
    pub height: u32,
    pub raw_data: Vec<u8>,
}

impl ImageDto {
    pub fn new(width: u32, height: u32, raw_data: Vec<u8>) -> Self {
        Self { width, height, raw_data }
    }

    pub fn from_image(image: RgbImage) -> Self {
        Self {
            width: image.width(),
            height: image.height(),
            raw_data: image.into_raw(),
        }
    }
}
