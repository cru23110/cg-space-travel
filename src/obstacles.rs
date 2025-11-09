use nalgebra_glm::Vec3;
use crate::geometry::Mesh;
use crate::color::Color;
use rand::Rng;

#[derive(Clone, Copy)]
pub enum ObstacleType {
    Cube,
    Sphere,
}

pub struct Obstacle {
    pub position: Vec3,
    pub obstacle_type: ObstacleType,
    pub size: f32,
    pub color: Color,
    pub active: bool,
    pub velocity: Vec3,
    pub shoot_timer: f32,
}

pub struct Projectile {
    pub position: Vec3,
    pub velocity: Vec3,
    pub active: bool,
    pub color: Color,
}

impl Obstacle {
    pub fn new(obstacle_type: ObstacleType, position: Vec3, size: f32) -> Self {
        let mut rng = rand::thread_rng();

        let color = match obstacle_type {
            ObstacleType::Cube => {
                let r = rng.gen_range(150..255);
                Color::new(r, 50, 50)
            },
            ObstacleType::Sphere => {
                let b = rng.gen_range(150..255);
                Color::new(100, 100, b)
            }
        };

        let velocity = Vec3::new(
            rng.gen_range(-0.5..0.5),
            rng.gen_range(-0.3..0.3),
            0.0
        );

        Obstacle {
            position,
            obstacle_type,
            size,
            color,
            active: true,
            velocity,
            shoot_timer: rng.gen_range(1.0..3.0),
        }
    }

    pub fn update(&mut self, ship_forward: Vec3, ship_pos: Vec3, delta_time: f32) -> Option<Projectile> {
        self.position -= ship_forward * 0.1 * delta_time * 60.0;
        self.position += self.velocity * delta_time * 60.0;

        if self.position.magnitude() > 50.0 {
            self.active = false;
        }

        self.shoot_timer -= delta_time;
        if self.shoot_timer <= 0.0 {
            self.shoot_timer = rand::thread_rng().gen_range(2.0..4.0);
            return Some(self.shoot(ship_pos));
        }

        None
    }

    pub fn shoot(&self, ship_pos: Vec3) -> Projectile {
        let direction = (ship_pos - self.position).normalize();

        Projectile {
            position: self.position.clone(),
            velocity: direction * 0.15,
            active: true,
            color: Color::new(255, 50, 50),
        }
    }

    pub fn check_collision(&self, ship_pos: Vec3, ship_size: f32) -> bool {
        let distance = (self.position - ship_pos).magnitude();
        distance < (self.size + ship_size)
    }
}

impl Projectile {
    pub fn update(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time * 60.0;

        if self.position.magnitude() > 50.0 {
            self.active = false;
        }
    }

    pub fn check_collision(&self, ship_pos: Vec3) -> bool {
        let distance = (self.position - ship_pos).magnitude();
        distance < 0.3
    }
}

pub struct ObstacleManager {
    pub obstacles: Vec<Obstacle>,
    pub projectiles: Vec<Projectile>,
    spawn_timer: f32,
    spawn_distance: f32,
}

impl ObstacleManager {
    pub fn new() -> Self {
        ObstacleManager {
            obstacles: Vec::new(),
            projectiles: Vec::new(),
            spawn_timer: 0.0,
            spawn_distance: 15.0,
        }
    }

    pub fn update(&mut self, ship_pos: Vec3, ship_forward: Vec3, delta_time: f32) {
        for obstacle in &mut self.obstacles {
            if let Some(projectile) = obstacle.update(ship_forward, ship_pos, delta_time) {
                self.projectiles.push(projectile);
            }
        }

        for projectile in &mut self.projectiles {
            projectile.update(delta_time);
        }

        self.obstacles.retain(|o| o.active);
        self.projectiles.retain(|p| p.active);

        self.spawn_timer -= delta_time;
        if self.spawn_timer <= 0.0 {
            self.spawn_obstacle(ship_pos, ship_forward);
            self.spawn_timer = 1.5;
        }
    }

    fn spawn_obstacle(&mut self, ship_pos: Vec3, ship_forward: Vec3) {
        let mut rng = rand::thread_rng();

        let offset_x = rng.gen_range(-3.0..3.0);
        let offset_y = rng.gen_range(-2.0..2.0);

        let right = Vec3::new(-ship_forward.z, 0.0, ship_forward.x).normalize();
        let up = ship_forward.cross(&right).normalize();

        let position = ship_pos
            + ship_forward * self.spawn_distance
            + right * offset_x
            + up * offset_y;

        let obstacle_type = if rng.gen_bool(0.6) {
            ObstacleType::Cube
        } else {
            ObstacleType::Sphere
        };

        let size = rng.gen_range(0.3..0.6);

        self.obstacles.push(Obstacle::new(obstacle_type, position, size));
    }

    pub fn check_collisions(&self, ship_pos: Vec3) -> bool {
        for obstacle in &self.obstacles {
            if obstacle.check_collision(ship_pos, 0.2) {
                println!("Hit obstacle at {:?}", obstacle.position);
                return true;
            }
        }

        for projectile in &self.projectiles {
            if projectile.check_collision(ship_pos) {
                println!("Hit by projectile at {:?}", projectile.position);
                return true;
            }
        }

        false
    }
}
