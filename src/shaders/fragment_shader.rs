use crate::fragment::Fragment;
use crate::color::Color;
use crate::uniforms::Uniforms;
use crate::shaders::planet_shaders::planet_shader;
use nalgebra_glm::Vec3;

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    if let Some(shader_type) = uniforms.planet_shader {
        planet_shader(fragment, shader_type)
    } else {
        let light_dir = Vec3::new(0.0, 0.0, -1.0).normalize();

        let intensity = fragment.normal.dot(&light_dir).max(0.0);
        let ambient = 0.3;
        let final_intensity = (ambient + (1.0 - ambient) * intensity).min(1.0);

        fragment.color * final_intensity
    }
}
