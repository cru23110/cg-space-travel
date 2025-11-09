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
mod warp;
mod skybox;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::time::Instant;
use std::f32::consts::PI;

use framebuffer::Framebuffer;
use camera::Camera;
use uniforms::{Uniforms, create_viewport_matrix, create_projection_matrix, create_model_matrix};
use pipeline::triangle_3d;
use celestial::{Planet, PlanetShader, Star, Ship};
use warp::WarpEffect;
use skybox::Skybox;

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

    let mut ship = Ship::new("assets/models/ship.obj")
        .unwrap_or_else(|e| {
            panic!("Failed to load ship model: {}", e);
        });
    ship.scale = 0.01;

    let mut sun = Star::new(1.5, Vec3::new(0.0, 0.0, 0.0));

    let mut planets = vec![
        Planet::new(PlanetShader::Rocky, 0.4, 4.0, 0.8),
        Planet::new(PlanetShader::Lava, 0.6, 6.0, 0.6),
        Planet::new(PlanetShader::Rocky, 0.7, 8.0, 0.5),
        Planet::new(PlanetShader::Rocky, 0.5, 10.0, 0.4),
        Planet::new(PlanetShader::Gaseous, 1.5, 14.0, 0.3),
        Planet::new(PlanetShader::Gaseous, 1.3, 18.0, 0.25),
        Planet::new(PlanetShader::Gaseous, 0.9, 22.0, 0.2),
        Planet::new(PlanetShader::Gaseous, 0.85, 26.0, 0.15),
    ];

    let mut warp_effect = WarpEffect::new();

    let skybox = Skybox::new(1000);

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
    let mut f_key_was_pressed = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed = start_time.elapsed().as_secs_f32();
        uniforms.time = elapsed;

        let camera_speed = if warp_effect.active { 0.5 } else { 0.1 };

        if window.is_key_down(Key::F) {
            if !f_key_was_pressed {
                warp_effect.toggle();
                f_key_was_pressed = true;
            }
        } else {
            f_key_was_pressed = false;
        }

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

        sun.update(0.016);

        for planet in &mut planets {
            planet.update(0.016);
        }

        ship.update(&camera);

        warp_effect.update(0.016, &camera);

        framebuffer.clear();

        skybox.render(&mut framebuffer, &uniforms);

        uniforms.model_matrix = create_model_matrix(
            sun.position,
            1.0,
            Vec3::new(sun.rotation, sun.rotation * 0.5, 0.0),
        );
        uniforms.is_star = true;
        uniforms.planet_shader = None;

        for i in (0..sun.mesh.vertices.len()).step_by(3) {
            if i + 2 < sun.mesh.vertices.len() {
                let v1 = &sun.mesh.vertices[i];
                let v2 = &sun.mesh.vertices[i + 1];
                let v3 = &sun.mesh.vertices[i + 2];

                triangle_3d(v1, v2, v3, &uniforms, &mut framebuffer);
            }
        }

        for planet in &planets {
            uniforms.model_matrix = create_model_matrix(
                planet.position,
                1.0,
                Vec3::new(planet.rotation, planet.rotation * 0.7, 0.0),
            );
            uniforms.is_star = false;
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

        uniforms.model_matrix = ship.get_model_matrix();
        uniforms.is_star = false;
        uniforms.planet_shader = None;

        for i in (0..ship.mesh.vertices.len()).step_by(3) {
            if i + 2 < ship.mesh.vertices.len() {
                let v1 = &ship.mesh.vertices[i];
                let v2 = &ship.mesh.vertices[i + 1];
                let v3 = &ship.mesh.vertices[i + 2];

                triangle_3d(v1, v2, v3, &uniforms, &mut framebuffer);
            }
        }

        for particle in &warp_effect.particles {
            let pos_4d = nalgebra_glm::vec3_to_vec4(&particle.position);
            let clip_pos = uniforms.projection_matrix * uniforms.view_matrix * pos_4d;

            if clip_pos.w > 0.0 {
                let ndc = clip_pos / clip_pos.w;

                if ndc.x >= -1.0 && ndc.x <= 1.0 && ndc.y >= -1.0 && ndc.y <= 1.0 && ndc.z >= 0.0 && ndc.z <= 1.0 {
                    let screen = uniforms.viewport_matrix * ndc;
                    let x = screen.x as usize;
                    let y = screen.y as usize;

                    if x < WIDTH && y < HEIGHT {
                        for dy in 0..8 {
                            for dx in 0..8 {
                                if x + dx < WIDTH && y + dy < HEIGHT {
                                    framebuffer.point_with_depth(x + dx, y + dy, ndc.z, &particle.color);
                                }
                            }
                        }
                    }
                }
            }
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
