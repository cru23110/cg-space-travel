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

        let offset_x = rng.gen_range(-0.3..0.3);
        let offset_y = rng.gen_range(-0.3..0.3);
        let offset_z = rng.gen_range(-0.5..0.1);

        let right = Vec3::new(-ship_forward.z, 0.0, ship_forward.x).normalize();
        let up = ship_forward.cross(&right).normalize();

        let position = ship_pos
            + ship_forward * offset_z
            + right * offset_x
            + up * offset_y;

        let velocity = -ship_forward * 8.0;

        let color_choice = rng.gen_range(0..3);
        let color = match color_choice {
            0 => Color::new(255, 255, 255),
            1 => Color::CYAN,
            _ => Color::MAGENTA,
        };

        WarpParticle {
            position,
            velocity,
            color,
            lifetime: 1.0,
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
}

impl WarpEffect {
    pub fn new() -> Self {
        WarpEffect {
            active: false,
            particles: Vec::new(),
            spawn_timer: 0.0,
        }
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
        if !self.active {
            self.particles.clear();
        }
    }

    pub fn update(&mut self, delta_time: f32, ship_pos: Vec3, ship_forward: Vec3) {
        if !self.active {
            return;
        }

        self.spawn_timer -= delta_time;
        if self.spawn_timer <= 0.0 {
            for _ in 0..20 {
                self.particles.push(WarpParticle::new(ship_pos, ship_forward));
            }
            self.spawn_timer = 0.02;
        }

        self.particles.retain_mut(|p| {
            p.update(delta_time);
            p.lifetime > 0.0
        });
    }
}
