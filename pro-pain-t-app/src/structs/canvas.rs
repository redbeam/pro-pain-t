use crate::structs::{color::Color, pixel::Pixel};

#[derive(Clone)]
#[allow(dead_code, unused_variables)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    content: Vec<Pixel>, // Two-dimensional - position of [x, y] is y * width + x
    background_color: Color,
}

impl Canvas {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let mut content: Vec<Pixel> = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                content.push(Pixel::new(x, y, background_color));
            }
        }

        Self {
            width,
            height,
            content,
            background_color,
        }
    }

    pub fn set_pixel(&mut self, pixel: Pixel) -> Result<(), String> {
        if pixel.x >= self.width || pixel.y >= self.height {
            return Err("Pixel out of bounds".to_string());
        }

        let index = (pixel.y * self.width + pixel.x) as usize;
        self.content[index] = pixel;

        Ok(())
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Result<&Pixel, String> {
        if x >= self.width || y >= self.height {
            return Err("Pixel out of bounds".to_string());
        }

        let index = (y * self.width + x) as usize;

        Ok(&self.content[index])
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width == self.width && new_height == self.height {
            return;
        }

        let mut new_content: Vec<Pixel> = Vec::with_capacity((new_width * new_height) as usize);

        for y in 0..new_height {
            for x in 0..new_width {
                if x < self.width && y < self.height {
                    let old_index = (y * self.width + x) as usize;
                    let mut pixel = self.content[old_index];
                    pixel.x = x;
                    pixel.y = y;
                    new_content.push(pixel);
                } else {
                    new_content.push(Pixel::new(x, y, self.background_color));
                }
            }
        }

        self.width = new_width;
        self.height = new_height;
        self.content = new_content;
    }
}
