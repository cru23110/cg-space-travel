use crate::fragment::Fragment;
use crate::color::Color;
use crate::celestial::PlanetShader;
use nalgebra_glm::Vec3;

fn simple_noise(p: Vec3) -> f32 {
    let x = (p.x * 50.0).sin() * (p.y * 50.0).cos();
    let y = (p.y * 50.0).sin() * (p.z * 50.0).cos();
    let z = (p.z * 50.0).sin() * (p.x * 50.0).cos();
    ((x + y + z) / 3.0).abs()
}

pub fn planet_shader(fragment: &Fragment, shader_type: PlanetShader) -> Color {
    let light_dir = Vec3::new(0.0, 0.0, -1.0).normalize();
    
    let intensity = fragment.normal.dot(&light_dir).max(0.0);
    let ambient = 0.2;
    let diffuse = (ambient + (1.0 - ambient) * intensity).min(1.0);
    
    match shader_type {
        PlanetShader::Rocky => rocky_shader(fragment, diffuse),
        PlanetShader::Gaseous => gaseous_shader(fragment, diffuse),
        PlanetShader::Lava => lava_shader(fragment, diffuse),
    }
}

fn rocky_shader(fragment: &Fragment, diffuse: f32) -> Color {
    let noise = simple_noise(fragment.normal);
    
    let ocean_color = Color::CYAN;
    let land_color = Color::new(0, 200, 100);
    
    let base_color = if noise > 0.5 {
        land_color
    } else {
        ocean_color
    };
    
    base_color * (0.3 + 0.7 * diffuse)
}

fn gaseous_shader(fragment: &Fragment, diffuse: f32) -> Color {
    let band = (fragment.normal.y * 10.0).sin() * 0.5 + 0.5;
    
    let color1 = Color::MAGENTA;
    let color2 = Color::PURPLE;
    
    let base_color = color1.lerp(&color2, band);
    
    base_color * (0.3 + 0.7 * diffuse)
}

fn lava_shader(fragment: &Fragment, diffuse: f32) -> Color {
    let noise = simple_noise(fragment.normal * 2.0);
    
    let dark_color = Color::new(20, 20, 20);
    let lava_color = Color::new(255, 100, 0);
    let bright_lava = Color::new(255, 0, 128);
    
    let lava_amount = if noise > 0.6 { 1.0 } else { 0.0 };
    let bright_amount = if noise > 0.8 { 1.0 } else { 0.0 };
    
    let base_color = if bright_amount > 0.5 {
        bright_lava
    } else if lava_amount > 0.5 {
        lava_color
    } else {
        dark_color
    };
    
    let emission = lava_amount * 0.3;
    base_color * (0.3 + 0.7 * diffuse + emission)
}
