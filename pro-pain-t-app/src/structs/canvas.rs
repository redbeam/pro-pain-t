pub struct Canvas {
    width: u32,
    height: u32,
    content: Vec<Pixel>, // Two-dimensional - position of [x, y] is y * width + height
    background_color: Color,
}

impl Canvas {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let retval = Canvas {
            width,
            height,
            content: Vec::new(width * height),
            background_color,
        };

        retval.content.foreach(|pixel| pixel.color = background_color);
        
        retval
    }

    pub fn set_pixel(x: u32, y: u32) -> Result<(), string> {
    }

    pub fn get_pixel(x: u32, y: u32) -> Pixel {
    }
}