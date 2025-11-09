use crate::geometry::{Mesh, create_sphere};
use nalgebra_glm::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum PlanetShader {
    Rocky,
    Gaseous,
    Lava,
}

pub struct Planet {
    pub position: Vec3,
    pub radius: f32,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub shader_type: PlanetShader,
    pub mesh: Mesh,
}

impl Planet {
    pub fn new(shader_type: PlanetShader, radius: f32, position: Vec3) -> Self {
        let mesh = create_sphere(radius, 12, 12);
        
        Planet {
            position,
            radius,
            rotation: 0.0,
            rotation_speed: 0.1,
            shader_type,
            mesh,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.rotation += self.rotation_speed * delta_time;
    }
}
