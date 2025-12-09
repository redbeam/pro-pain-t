use crate::structs::color::Color;
#[allow(dead_code, unused_variables)]
#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    x: u32,
    y: u32,
    pub color: Color,
}

impl Pixel {
    pub fn new(x: u32, y: u32, color: Color) -> Self {
        Self { x, y, color }
    }
}
