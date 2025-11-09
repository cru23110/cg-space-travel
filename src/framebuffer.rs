use crate::color::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: Color::DARK_BG,
        }
    }

    pub fn clear(&mut self) {
        let bg_hex = self.background_color.to_hex();
        for pixel in self.buffer.iter_mut() {
            *pixel = bg_hex;
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: &Color) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = color.to_hex();
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
}
