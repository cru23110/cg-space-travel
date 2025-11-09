use crate::fragment::Fragment;
use crate::color::Color;
use crate::uniforms::Uniforms;
use nalgebra_glm::Vec3;
use noise::NoiseFn;

pub fn sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let light_dir = Vec3::new(0.0, 0.0, -1.0).normalize();

    let base_intensity = fragment.normal.dot(&light_dir).max(0.0);

    let noise_coords = [
        (fragment.normal.x * 3.0 + uniforms.time * 0.2) as f64,
        (fragment.normal.y * 3.0) as f64,
        (fragment.normal.z * 3.0) as f64,
    ];

    let noise_value = uniforms.noise_generator.get(noise_coords);
    let turbulence = (noise_value as f32 * 0.5 + 0.5) * 0.3;

    let dist_from_center = (1.0 - base_intensity).max(0.0);

    let core_color = Color::new(255, 200, 0);
    let mid_color = Color::new(255, 100, 0);
    let edge_color = Color::new(255, 0, 128);

    let t = (dist_from_center + turbulence).clamp(0.0, 1.0);

    let color = if t < 0.5 {
        core_color.lerp(&mid_color, t * 2.0)
    } else {
        mid_color.lerp(&edge_color, (t - 0.5) * 2.0)
    };

    let pulse = ((uniforms.time * 1.5).sin() * 0.15 + 1.0).max(0.85);

    color * pulse
}
