use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Vertex2D {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

impl Vertex2D {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Vertex2D { x, y, color }
    }
}
