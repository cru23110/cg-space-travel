use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn from_float(r: f32, g: f32, b: f32) -> Self {
        Color {
            r: (r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (b.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
        }
    }

    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const RED: Color = Color::new(255, 0, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const BLUE: Color = Color::new(0, 0, 255);
    
    // Retro colors (Synthwave palette)
    pub const MAGENTA: Color = Color::new(255, 0, 110);
    pub const CYAN: Color = Color::new(0, 245, 255);
    pub const NEON_YELLOW: Color = Color::new(255, 234, 0);
    pub const PURPLE: Color = Color::new(139, 0, 255);
    pub const DARK_BG: Color = Color::new(10, 10, 31);
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        Color::from_float(
            (self.r as f32 / 255.0) * scalar,
            (self.g as f32 / 255.0) * scalar,
            (self.b as f32 / 255.0) * scalar,
        )
    }
}
