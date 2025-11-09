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

        Obstacle {
            position,
            obstacle_type,
            size,
            color,
            active: true,
        }
    }

    pub fn update(&mut self, ship_forward: Vec3, delta_time: f32) {
        self.position -= ship_forward * 0.1 * delta_time * 60.0;

        if self.position.magnitude() > 50.0 {
            self.active = false;
        }
    }

    pub fn check_collision(&self, ship_pos: Vec3, ship_size: f32) -> bool {
        let distance = (self.position - ship_pos).magnitude();
        distance < (self.size + ship_size)
    }
}

pub struct ObstacleManager {
    pub obstacles: Vec<Obstacle>,
    spawn_timer: f32,
    spawn_distance: f32,
}

impl ObstacleManager {
    pub fn new() -> Self {
        ObstacleManager {
            obstacles: Vec::new(),
            spawn_timer: 0.0,
            spawn_distance: 15.0,
        }
    }

    pub fn update(&mut self, ship_pos: Vec3, ship_forward: Vec3, delta_time: f32) {
        for obstacle in &mut self.obstacles {
            obstacle.update(ship_forward, delta_time);
        }

        self.obstacles.retain(|o| o.active);

        self.spawn_timer -= delta_time;
        if self.spawn_timer <= 0.0 {
            self.spawn_obstacle(ship_pos, ship_forward);
            self.spawn_timer = 2.0;
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
            if obstacle.check_collision(ship_pos, 0.15) {
                return true;
            }
        }
        false
    }
}
