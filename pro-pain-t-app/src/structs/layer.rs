use leptos::prelude::RwSignal;

use image::RgbImage;
use serde::{Deserialize, Serialize};
use crate::structs::{canvas::Canvas, color::Color};

#[derive(Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: usize,
    pub title: RwSignal<String>,
    pub is_locked: bool,
    pub canvas: Canvas,
    pub is_visible: bool,
}

impl Layer {
    pub fn new(id: usize, title: String, width: u32, height: u32, background_color: Color) -> Self {
        Self {
            id,
            title: RwSignal::new(title),
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
}
