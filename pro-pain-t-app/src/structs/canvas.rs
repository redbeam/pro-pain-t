use image::RgbImage;
use serde::{Deserialize, Serialize};
use crate::structs::{color::Color, pixel::Pixel};

#[derive(Clone, Serialize, Deserialize)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub content: Vec<u8>, // One-dimensional - rgba bytes in order
    pub background_color: Color,
}

impl Canvas {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {

        let pixel = [
            background_color.r,
            background_color.g,
            background_color.b,
            (background_color.alpha * 255.0) as u8,
        ];

        let capacity = width
            .checked_mul(height * 4)
            .expect("canvas dimensions too large: width * height overflowed u32");

        let mut content = Vec::with_capacity(capacity as usize);

        for _ in 0..width * height {
            content.extend_from_slice(&pixel);
        }

        Self {
            width,
            height,
            content,
            background_color,
        }
    }

    pub fn from_image(image: &RgbImage, background_color: Color) -> Self {
        let mut out = Vec::with_capacity((image.width() * image.height() * 4) as usize);

        for (_x, _y, pixel) in image.enumerate_pixels() {
            out.push(pixel[0]); // R
            out.push(pixel[1]); // G
            out.push(pixel[2]); // B
            out.push(255);      // A
        };
 

        Self {
            width: image.width(),
            height: image.height(),
            content: out,
            background_color,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, rgba: [u8; 4]) -> Result<(), &'static str> {
        if x >= self.width || y >= self.height {
            return Err("pixel out of bounds");
        }

        let idx = ((y * self.width + x) * 4) as usize;
        self.content[idx..idx + 4].copy_from_slice(&rgba);
        Ok(())
    }
    
    pub fn get_pixel(&self, x: u32, y: u32) -> Result<Pixel, String> {
        if x >= self.width || y >= self.height {
            return Err("Pixel out of bounds".to_string());
        }

        let index = ((y * self.width + x) * 4) as usize;

        let r = self.content[index];
        let g = self.content[index + 1];
        let b = self.content[index + 2];
        let alpha = self.content[index + 3] as f32 / 255.0;

        Ok(Pixel {
            x,
            y,
            color: Color { r, g, b, alpha },
        })
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
    if new_width == self.width && new_height == self.height {
        return;
    }

    let new_len = (new_width * new_height * 4) as usize;
    let mut new_content = vec![0u8; new_len];

    let copy_width = self.width.min(new_width);
    let copy_height = self.height.min(new_height);

    for y in 0..copy_height {
        let old_row = (y * self.width * 4) as usize;
        let new_row = (y * new_width * 4) as usize;

        let bytes = (copy_width * 4) as usize;
        new_content[new_row..new_row + bytes]
            .copy_from_slice(&self.content[old_row..old_row + bytes]);
    }

    let background_pixel = [
        self.background_color.r,
        self.background_color.g,
        self.background_color.b,
        (self.background_color.alpha * 255.0) as u8,
    ];

    for y in 0..new_height {
        for x in 0..new_width {
            if x < copy_width && y < copy_height {
                continue;
            }
            let idx = ((y * new_width + x) * 4) as usize;
            new_content[idx..idx + 4].copy_from_slice(&background_pixel);
        }
    }

    self.width = new_width;
    self.height = new_height;
    self.content = new_content;
}


    pub fn clear(&mut self) {
        let pixel = [
            self.background_color.r,
            self.background_color.g,
            self.background_color.b,
            (self.background_color.alpha * 255.0) as u8,
        ];

        let capacity = self.width
            .checked_mul(self.height * 4)
            .expect("canvas dimensions too large: width * height overflowed u32");

        self.content = Vec::with_capacity(capacity as usize);

        for _ in 0..self.width * self.height {
            self.content.extend_from_slice(&pixel);
        }
    }
}
