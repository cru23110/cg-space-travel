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
    pub orbit_radius: f32,
    pub orbit_angle: f32,
    pub orbit_speed: f32,
    pub shader_type: PlanetShader,
    pub mesh: Mesh,
}

impl Planet {
    pub fn new(shader_type: PlanetShader, radius: f32, orbit_radius: f32, orbit_speed: f32) -> Self {
        let mesh = create_sphere(radius, 12, 12);

        let orbit_angle: f32 = 0.0;
        let position = Vec3::new(
            orbit_radius * orbit_angle.cos(),
            0.0,
            orbit_radius * orbit_angle.sin(),
        );

        Planet {
            position,
            radius,
            rotation: 0.0,
            rotation_speed: 0.1,
            orbit_radius,
            orbit_angle,
            orbit_speed,
            shader_type,
            mesh,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.rotation += self.rotation_speed * delta_time;

        self.orbit_angle += self.orbit_speed * delta_time;

        self.position.x = self.orbit_radius * self.orbit_angle.cos();
        self.position.z = self.orbit_radius * self.orbit_angle.sin();
        self.position.y = 0.0;
    }
}
