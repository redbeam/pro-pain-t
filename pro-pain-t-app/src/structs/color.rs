#[derive(Clone, Copy, Debug)]
#[allow(dead_code, unused_variables)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    alpha: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        Self { r, g, b, alpha }
    }
}
