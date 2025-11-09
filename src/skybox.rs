use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::uniforms::Uniforms;
use nalgebra_glm::Vec3;
use rand::Rng;

pub struct Star {
    pub direction: Vec3,
    pub brightness: u8,
    pub size: u8,
}

pub struct Skybox {
    stars: Vec<Star>,
}

impl Skybox {
    pub fn new(star_count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut stars = Vec::new();

        for _ in 0..star_count {
            let theta = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
            let phi = rng.gen_range(0.0..std::f32::consts::PI);

            let direction = Vec3::new(
                phi.sin() * theta.cos(),
                phi.sin() * theta.sin(),
                phi.cos(),
            ).normalize();

            let brightness = rng.gen_range(150..255);
            let size = if rng.gen_bool(0.1) { 2 } else { 1 };

            stars.push(Star {
                direction,
                brightness,
                size,
            });
        }

        Skybox { stars }
    }

    pub fn render(&self, framebuffer: &mut Framebuffer, uniforms: &Uniforms) {
        for star in &self.stars {
            let view_dir = uniforms.view_matrix * nalgebra_glm::vec3_to_vec4(&star.direction);
            let clip_pos = uniforms.projection_matrix * view_dir;

            if clip_pos.w > 0.0 {
                let ndc = clip_pos / clip_pos.w;

                if ndc.x >= -1.0 && ndc.x <= 1.0 && ndc.y >= -1.0 && ndc.y <= 1.0 {
                    let screen = uniforms.viewport_matrix * ndc;
                    let x = screen.x as usize;
                    let y = screen.y as usize;

                    let color = Color::new(star.brightness, star.brightness, star.brightness);

                    if x < framebuffer.width && y < framebuffer.height {
                        for dy in 0..star.size {
                            for dx in 0..star.size {
                                let px = x + dx as usize;
                                let py = y + dy as usize;
                                if px < framebuffer.width && py < framebuffer.height {
                                    framebuffer.point_with_depth(px, py, 1.0, &color);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
