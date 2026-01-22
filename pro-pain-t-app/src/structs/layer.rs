use image::RgbImage;
use serde::{Deserialize, Serialize};
use crate::structs::{canvas::Canvas, color::Color};

#[derive(Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: usize,
    pub title: String,
    pub is_locked: bool,
    pub canvas: Canvas,
    pub is_visible: bool,
}

impl Layer {
    pub fn new(id: usize, title: impl Into<String>, width: u32, height: u32, background_color: Color) -> Self {
        Self {
            id,
            title: title.into(),
            is_locked: false,
            canvas: Canvas::new(width, height, background_color),
            is_visible: true,
        }
    }

    pub fn from_image(image: &RgbImage, id: usize, title: impl Into<String>, background_color: Color) -> Self {
        Self {
            id,
            title: title.into(),
            is_locked: false,
            canvas: Canvas::from_image(image, background_color),
            is_visible: true,
        }
    }

    pub fn resize_canvas(&mut self, new_width: u32, new_height: u32) {
        self.canvas.resize(new_width, new_height);
    }

    pub fn to_rgba(&self) -> (Vec<u8>, u32, u32) {
        let canvas = &self.canvas;
        let width = canvas.width;
        let height = canvas.height;

        let mut out = vec![0u8; (width * height * 4) as usize];

        for pixel in &canvas.content {
            let i = ((pixel.y * width + pixel.x) * 4) as usize;

            out[i]     = pixel.color.r;
            out[i + 1] = pixel.color.g;
            out[i + 2] = pixel.color.b;
            out[i + 3] = (pixel.color.alpha * 255.0) as u8;
        }

        (out, width, height)
    }
}
