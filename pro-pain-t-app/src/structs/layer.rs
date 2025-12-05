pub struct Layer {
    is_locked: bool,
    canvas: Canvas,
    is_visible: bool,
}

impl Layer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        Layer {
            is_locked: false,
            canvas: Canvas::new(width, height, background_color),
            is_visible: true,
        }
    } 
}