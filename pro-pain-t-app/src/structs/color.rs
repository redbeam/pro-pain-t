use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub alpha: f32,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, alpha: f32) -> Self {
        Self { r, g, b, alpha }
    }

    pub fn default_white() -> Self {
        Self::new(255, 255, 255, 1.0)
    }

    pub fn default_black() -> Self {
        Self::new(0, 0, 0, 1.0)
    }
}
