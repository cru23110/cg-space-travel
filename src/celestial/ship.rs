use crate::geometry::{Mesh, load_obj};
use crate::camera::Camera;
use nalgebra_glm::{Vec3, Mat4};
use std::f32::consts::PI;

pub struct Ship {
    pub mesh: Mesh,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: f32,
}

impl Ship {
    pub fn new(obj_path: &str) -> Result<Self, String> {
        let mesh = load_obj(obj_path)?;

        Ok(Ship {
            mesh,
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: 1.0,
        })
    }

    pub fn update(&mut self, camera: &Camera) {
        let forward = camera.forward();
        let right = camera.right();
        let up = Vec3::new(0.0, 1.0, 0.0);

        let offset_forward = 2.0;
        let offset_right = 0.0;
        let offset_up = -0.5;

        self.position = camera.eye
            + forward * offset_forward
            + right * offset_right
            + up * offset_up;

        let yaw = forward.z.atan2(forward.x);
        self.rotation.y = yaw;
    }

    pub fn get_model_matrix(&self) -> Mat4 {
        let translation = nalgebra_glm::translate(&Mat4::identity(), &self.position);
        let scale_mat = nalgebra_glm::scale(&Mat4::identity(), &Vec3::new(self.scale, self.scale, self.scale));
        
        let rotation_x = nalgebra_glm::rotate(&Mat4::identity(), self.rotation.x, &Vec3::new(1.0, 0.0, 0.0));
        let rotation_y = nalgebra_glm::rotate(&Mat4::identity(), self.rotation.y, &Vec3::new(0.0, 1.0, 0.0));
        let rotation_z = nalgebra_glm::rotate(&Mat4::identity(), self.rotation.z, &Vec3::new(0.0, 0.0, 1.0));
        
        translation * rotation_z * rotation_y * rotation_x * scale_mat
    }
}
