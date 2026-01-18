use image::Rgb;
use crate::structs::color::Color;

#[allow(dead_code, unused_variables)]
#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: Color,
}

impl Pixel {
    pub fn new(x: u32, y: u32, color: Color) -> Self {
        Self { x, y, color }
    }

    pub fn from_rgb(x: u32, y: u32, color_rgb: Rgb<u8>) -> Self {
        Self {
            x,
            y,
            color: Color::new(color_rgb.0[0], color_rgb.0[1], color_rgb.0[2], 1.0),
        }
    }
}
