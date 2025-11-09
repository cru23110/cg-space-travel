use crate::color::Color;
use nalgebra_glm::Vec3;
use rand::Rng;

pub struct WarpParticle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub color: Color,
    pub lifetime: f32,
}

impl WarpParticle {
    pub fn new(ship_pos: Vec3, ship_forward: Vec3) -> Self {
        let mut rng = rand::thread_rng();

        let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        let radius = rng.gen_range(0.05..0.15);
        let offset_z = rng.gen_range(-0.1..0.1);

        let right = Vec3::new(-ship_forward.z, 0.0, ship_forward.x).normalize();
        let up = ship_forward.cross(&right).normalize();

        let offset_x = angle.cos() * radius;
        let offset_y = angle.sin() * radius;

        let position = ship_pos
            + ship_forward * offset_z
            + right * offset_x
            + up * offset_y;

        let velocity = -ship_forward * 3.0;

        let color_choice = rng.gen_range(0..10);
        let color = match color_choice {
            0..=6 => Color::new(0, 150, 255),
            7..=8 => Color::new(100, 200, 255),
            _ => Color::new(255, 255, 255),
        };

        WarpParticle {
            position,
            velocity,
            color,
            lifetime: 0.5,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
        self.lifetime -= delta_time;
    }
}

pub struct WarpEffect {
    pub active: bool,
    pub particles: Vec<WarpParticle>,
    spawn_timer: f32,
    pub intensity: f32,
    duration: f32,
}

impl WarpEffect {
    pub fn new() -> Self {
        WarpEffect {
            active: false,
            particles: Vec::new(),
            spawn_timer: 0.0,
            intensity: 0.0,
            duration: 0.0,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.duration = 2.0;
        self.intensity = 1.0;
    }

    pub fn is_boosting(&self) -> bool {
        self.intensity > 0.3
    }

    pub fn update(&mut self, delta_time: f32, ship_pos: Vec3, ship_forward: Vec3) {
        if self.active {
            self.duration -= delta_time;
            if self.duration <= 0.0 {
                self.active = false;
                self.intensity = 0.0;
            } else {
                self.intensity = (self.duration / 2.0).min(1.0);
            }
        }

        if self.active {
            self.spawn_timer -= delta_time;
            if self.spawn_timer <= 0.0 {
                let particle_count = (50.0 * self.intensity) as i32;
                for _ in 0..particle_count {
                    self.particles.push(WarpParticle::new(ship_pos, ship_forward));
                }
                self.spawn_timer = 0.01;
            }
        }

        self.particles.retain_mut(|p| {
            p.update(delta_time);
            p.lifetime > 0.0
        });
    }
}
