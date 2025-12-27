use crate::structs::{canvas::Canvas, color::Color};

#[derive(Clone)]
#[allow(dead_code, unused_variables)]
pub struct Layer {
    id: usize,
    title: String,
    is_locked: bool,
    canvas: Canvas,
    is_visible: bool,
}

impl Layer {
    pub fn new(id: usize, title: String, width: u32, height: u32, background_color: Color) -> Self {
        Self {
            id,
            title,
            is_locked: false,
            canvas: Canvas::new(width, height, background_color),
            is_visible: true,
        }
    }
}
