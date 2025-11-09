mod color;
mod framebuffer;
mod vertex;
mod fragment;
mod pipeline;
mod uniforms;
mod camera;
mod shaders;
mod geometry;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::time::Instant;
use std::f32::consts::PI;

use framebuffer::Framebuffer;
use camera::Camera;
use uniforms::{Uniforms, create_viewport_matrix, create_projection_matrix, create_model_matrix};
use geometry::create_cube;
use pipeline::triangle_3d;
use vertex::Vertex;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    
    let mut window = Window::new(
        "Space Travel - Software Renderer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Unable to create window: {}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, -5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let cube_vertices = create_cube();

    let mut uniforms = Uniforms::new();
    uniforms.projection_matrix = create_projection_matrix(
        45.0 * PI / 180.0,
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        100.0,
    );
    uniforms.view_matrix = camera.get_view_matrix();
    uniforms.viewport_matrix = create_viewport_matrix(WIDTH as f32, HEIGHT as f32);

    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed = start_time.elapsed().as_secs_f32();
        uniforms.time = elapsed;

        uniforms.model_matrix = create_model_matrix(
            Vec3::new(0.0, 0.0, 0.0),
            1.0,
            Vec3::new(elapsed * 0.5, elapsed * 0.3, 0.0),
        );

        framebuffer.clear();

        for i in (0..cube_vertices.len()).step_by(3) {
            if i + 2 < cube_vertices.len() {
                triangle_3d(
                    &cube_vertices[i],
                    &cube_vertices[i + 1],
                    &cube_vertices[i + 2],
                    &uniforms,
                    &mut framebuffer,
                );
            }
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
