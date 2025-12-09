use crate::structs::{canvas::Canvas, color::Color};

#[allow(dead_code, unused_variables)]
pub struct Layer {
    id: usize,
    is_locked: bool,
    canvas: Canvas,
    is_visible: bool,
}

impl Layer {
    pub fn new(id: usize, width: u32, height: u32, background_color: Color) -> Self {
        Self {
            id,
            is_locked: false,
            canvas: Canvas::new(width, height, background_color),
            is_visible: true,
        }
    }
}
