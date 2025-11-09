use crate::color::Color;
use nalgebra_glm::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Fragment {
    pub position: (usize, usize),
    pub depth: f32,
    pub color: Color,
    pub normal: Vec3,
    pub intensity: f32,
}

impl Fragment {
    pub fn new(x: usize, y: usize, depth: f32) -> Self {
        Fragment {
            position: (x, y),
            depth,
            color: Color::BLACK,
            normal: Vec3::new(0.0, 0.0, 0.0),
            intensity: 1.0,
        }
    }
}
