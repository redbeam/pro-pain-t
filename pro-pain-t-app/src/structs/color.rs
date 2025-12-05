pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    alpha: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        Color {
            r,
            g,
            b,
            alpha,
        }
    }
}