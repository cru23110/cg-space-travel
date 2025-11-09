use crate::fragment::Fragment;
use crate::color::Color;
use nalgebra_glm::Vec3;

pub fn fragment_shader(fragment: &Fragment) -> Color {
    let light_dir = Vec3::new(0.0, 0.0, -1.0).normalize();
    
    let intensity = fragment.normal.dot(&light_dir).max(0.0);
    let ambient = 0.3;
    let final_intensity = (ambient + (1.0 - ambient) * intensity).min(1.0);
    
    fragment.color * final_intensity
}
