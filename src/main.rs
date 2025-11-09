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
mod phase_manager;
mod obstacles;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::time::Instant;
use std::f32::consts::PI;

use framebuffer::Framebuffer;
use camera::Camera;
use uniforms::{Uniforms, create_viewport_matrix, create_projection_matrix};
use pipeline::triangle_3d;
use celestial::Ship;
use warp::WarpEffect;
use skybox::Skybox;
use phase_manager::PhaseManager;
use obstacles::ObstacleManager;
use geometry::create_cube;

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
    ship.scale = 0.08;

    let mut phase_manager = PhaseManager::new();

    let mut warp_effect = WarpEffect::new();

    let skybox = Skybox::new(1000);

    let mut obstacle_manager = ObstacleManager::new();

    let cube_mesh = create_cube();

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

        if window.is_key_down(Key::F) {
            if !f_key_was_pressed {
                warp_effect.activate();
                f_key_was_pressed = true;
            }
        } else {
            f_key_was_pressed = false;
        }

        let current_phase = phase_manager.current_phase();

        let mut pitch_input = 0.0;
        let mut roll_input = 0.0;

        match current_phase.phase_type {
            phase_manager::PhaseType::TopDown | phase_manager::PhaseType::TopDownShooter => {
                if window.is_key_down(Key::W) {
                    pitch_input -= 1.0;
                }
                if window.is_key_down(Key::S) {
                    pitch_input += 1.0;
                }
                if window.is_key_down(Key::A) {
                    roll_input -= 1.0;
                }
                if window.is_key_down(Key::D) {
                    roll_input += 1.0;
                }
            },
            phase_manager::PhaseType::Side => {
                if window.is_key_down(Key::W) {
                    pitch_input -= 1.0;
                }
                if window.is_key_down(Key::S) {
                    pitch_input += 1.0;
                }
                if window.is_key_down(Key::A) {
                    roll_input -= 1.0;
                }
                if window.is_key_down(Key::D) {
                    roll_input += 1.0;
                }
            }
        }

        ship.apply_input(pitch_input, roll_input);

        let ship_forward = ship.get_forward_direction();
        warp_effect.update(0.016, ship.position, ship_forward);

        ship.update_physics(0.016, warp_effect.intensity);

        obstacle_manager.update(ship.position, ship_forward, 0.016);

        if obstacle_manager.check_collisions(ship.position) {
            println!("Collision detected!");
        }

        if window.is_key_down(Key::Space) {
            phase_manager.next_phase();
        }

        phase_manager.update(0.016);
        phase_manager.current_phase().setup_camera(&mut camera, ship.position);

        uniforms.view_matrix = camera.get_view_matrix();

        framebuffer.clear();

        skybox.render(&mut framebuffer, &uniforms);

        for obstacle in &obstacle_manager.obstacles {
            use uniforms::create_model_matrix;
            uniforms.model_matrix = create_model_matrix(
                obstacle.position,
                obstacle.size,
                Vec3::new(0.0, 0.0, 0.0)
            );
            uniforms.is_star = false;
            uniforms.planet_shader = None;

            for i in (0..cube_mesh.len()).step_by(3) {
                if i + 2 < cube_mesh.len() {
                    let v1 = &cube_mesh[i];
                    let v2 = &cube_mesh[i + 1];
                    let v3 = &cube_mesh[i + 2];

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
                        for dy in 0..16 {
                            for dx in 0..16 {
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
