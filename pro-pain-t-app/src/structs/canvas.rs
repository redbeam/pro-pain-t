use image::RgbImage;
use serde::{Deserialize, Serialize};
use crate::structs::{color::Color, pixel::Pixel};

#[derive(Clone, Serialize, Deserialize)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    content: Vec<Pixel>, // Two-dimensional - position of [x, y] is y * width + x
    pub background_color: Color,
}

impl Canvas {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let capacity = width
            .checked_mul(height)
            .expect("canvas dimensions too large: width * height overflowed u32");

        let mut content: Vec<Pixel> = Vec::with_capacity(capacity as usize);

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

    pub fn from_image(image: &RgbImage, background_color: Color) -> Self {
        let content = image
            .enumerate_pixels()
            .into_iter()
            .map(|pixel| Pixel::from_rgb(pixel.0, pixel.1, *pixel.2))
            .collect::<Vec<Pixel>>();

        Self {
            width: image.width(),
            height: image.height(),
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

        let capacity = new_width
            .checked_mul(new_height)
            .expect("Canvas dimensions too large: width * height overflowed u32");

        let mut new_content: Vec<Pixel> = Vec::with_capacity(capacity as usize);

        for y in 0..new_height {
            for x in 0..new_width {
                if x < self.width && y < self.height {
                    let old_index = (y * self.width + x) as usize;
                    let pixel = self.content[old_index];
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
