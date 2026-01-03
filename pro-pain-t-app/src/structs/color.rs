#[derive(Clone, Copy, Debug)]
#[allow(dead_code, unused_variables)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        Self { r, g, b, alpha }
    }
}
