use nalgebra_glm::{Mat4, Vec3};

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: f32,
}

impl Uniforms {
    pub fn new() -> Self {
        Uniforms {
            model_matrix: Mat4::identity(),
            view_matrix: Mat4::identity(),
            projection_matrix: Mat4::identity(),
            viewport_matrix: Mat4::identity(),
            time: 0.0,
        }
    }
}

pub fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

pub fn create_projection_matrix(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    nalgebra_glm::perspective(aspect, fov, near, far)
}

pub fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let translate = nalgebra_glm::translate(&Mat4::identity(), &translation);
    let scale_mat = nalgebra_glm::scale(&Mat4::identity(), &Vec3::new(scale, scale, scale));
    let rotate_x = nalgebra_glm::rotate(&Mat4::identity(), rotation.x, &Vec3::new(1.0, 0.0, 0.0));
    let rotate_y = nalgebra_glm::rotate(&Mat4::identity(), rotation.y, &Vec3::new(0.0, 1.0, 0.0));
    let rotate_z = nalgebra_glm::rotate(&Mat4::identity(), rotation.z, &Vec3::new(0.0, 0.0, 1.0));
    
    translate * rotate_z * rotate_y * rotate_x * scale_mat
}
