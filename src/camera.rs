use nalgebra_glm::{Vec3, Mat4, look_at};

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera { eye, center, up }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        look_at(&self.eye, &self.center, &self.up)
    }
}
