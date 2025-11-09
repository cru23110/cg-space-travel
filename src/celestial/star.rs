use crate::geometry::{Mesh, create_sphere};
use nalgebra_glm::Vec3;

pub struct Star {
    pub position: Vec3,
    pub radius: f32,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub mesh: Mesh,
}

impl Star {
    pub fn new(radius: f32, position: Vec3) -> Self {
        let mesh = create_sphere(radius, 16, 16);

        Star {
            position,
            radius,
            rotation: 0.0,
            rotation_speed: 0.05,
            mesh,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.rotation += self.rotation_speed * delta_time;
    }
}
