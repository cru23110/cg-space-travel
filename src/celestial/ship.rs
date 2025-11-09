use crate::geometry::{Mesh, load_obj};
use crate::camera::Camera;
use nalgebra_glm::{Vec3, Mat4};
use std::f32::consts::PI;

pub struct Ship {
    pub mesh: Mesh,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: f32,
    pub velocity: Vec3,
    pub target_rotation: Vec3,
    pub acceleration: f32,
    pub max_speed: f32,
    pub rotation_speed: f32,
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
    pub forward_speed: f32,
}

impl Ship {
    pub fn new(obj_path: &str) -> Result<Self, String> {
        let mesh = load_obj(obj_path)?;

        Ok(Ship {
            mesh,
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: 1.0,
            velocity: Vec3::new(0.0, 0.0, 0.0),
            target_rotation: Vec3::new(0.0, 0.0, 0.0),
            acceleration: 0.03,
            max_speed: 0.25,
            rotation_speed: 0.08,
            pitch: 0.0,
            roll: 0.0,
            yaw: 0.0,
            forward_speed: 0.1,
        })
    }

    pub fn apply_input(&mut self, pitch_input: f32, roll_input: f32) {
        self.pitch += pitch_input * self.rotation_speed;
        self.roll += roll_input * self.rotation_speed;
        self.yaw += roll_input * self.rotation_speed * 0.5;

        self.pitch = self.pitch.clamp(-PI / 3.0, PI / 3.0);
        self.roll = self.roll.clamp(-PI / 4.0, PI / 4.0);

        self.pitch *= 0.95;
        self.roll *= 0.95;
    }

    pub fn update_physics(&mut self, delta_time: f32) {
        self.rotation.x = -PI / 2.0 + self.pitch;
        self.rotation.y = self.yaw;
        self.rotation.z = self.roll;

        let cos_pitch = self.pitch.cos();
        let sin_pitch = self.pitch.sin();
        let cos_yaw = self.yaw.cos();
        let sin_yaw = self.yaw.sin();

        let forward = Vec3::new(
            sin_yaw * cos_pitch,
            sin_pitch,
            -cos_yaw * cos_pitch
        );

        self.position += forward * self.forward_speed * delta_time * 60.0;
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
