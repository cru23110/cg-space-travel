mod color;
mod framebuffer;
mod vertex;
mod fragment;
mod pipeline;
mod uniforms;
mod camera;
mod shaders;
mod geometry;
mod celestial;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::time::Instant;
use std::f32::consts::PI;

use framebuffer::Framebuffer;
use camera::Camera;
use uniforms::{Uniforms, create_viewport_matrix, create_projection_matrix, create_model_matrix};
use pipeline::triangle_3d;
use celestial::{Ship, Planet, PlanetShader};

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

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, -10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );


    let mut planets = vec![
        Planet::new(PlanetShader::Rocky, 1.0, Vec3::new(-3.0, 0.0, 0.0)),
        Planet::new(PlanetShader::Gaseous, 1.2, Vec3::new(0.0, 0.0, 0.0)),
        Planet::new(PlanetShader::Lava, 0.8, Vec3::new(3.0, 0.0, 0.0)),
    ];  // Much smaller scale

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

        let camera_speed = 0.1;

        if window.is_key_down(Key::W) {
            camera.move_forward(camera_speed);
        }
        if window.is_key_down(Key::S) {
            camera.move_forward(-camera_speed);
        }
        if window.is_key_down(Key::A) {
            camera.move_right(-camera_speed);
        }
        if window.is_key_down(Key::D) {
            camera.move_right(camera_speed);
        }
        if window.is_key_down(Key::Q) {
            camera.move_up(camera_speed);
        }
        if window.is_key_down(Key::E) {
            camera.move_up(-camera_speed);
        }

        uniforms.view_matrix = camera.get_view_matrix();

        for planet in &mut planets {
            planet.update(0.016);
        }

        framebuffer.clear();

        for planet in &planets {
            uniforms.model_matrix = create_model_matrix(
                planet.position,
                1.0,
                Vec3::new(planet.rotation, planet.rotation * 0.7, 0.0),
            );
            uniforms.planet_shader = Some(planet.shader_type);

            for i in (0..planet.mesh.vertices.len()).step_by(3) {
                if i + 2 < planet.mesh.vertices.len() {
                    let v1 = &planet.mesh.vertices[i];
                    let v2 = &planet.mesh.vertices[i + 1];
                    let v3 = &planet.mesh.vertices[i + 2];

                    triangle_3d(v1, v2, v3, &uniforms, &mut framebuffer);
                }
            }
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
