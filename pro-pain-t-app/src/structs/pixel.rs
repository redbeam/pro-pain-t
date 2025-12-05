pub struct Pixel {
    x: u32,
    y: u32,
    color: Color,
}

impl Pixel {
    pub fn new(x: u32, y: u32, color: Color) -> Self {
        Pixel {
            x,
            y,
            color,
        }
    }
}