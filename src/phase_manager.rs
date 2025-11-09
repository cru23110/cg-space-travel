use nalgebra_glm::Vec3;
use crate::camera::Camera;

#[derive(Clone, Copy, PartialEq)]
pub enum PhaseType {
    TopDown,
    Side,
    TopDownShooter,
}

pub struct Phase {
    pub phase_type: PhaseType,
    pub camera_position: Vec3,
    pub camera_target: Vec3,
    pub camera_up: Vec3,
    pub completed: bool,
}

impl Phase {
    pub fn new(phase_type: PhaseType) -> Self {
        let (camera_position, camera_target, camera_up) = match phase_type {
            PhaseType::TopDown => (
                Vec3::new(0.0, 20.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
            ),
            PhaseType::Side => (
                Vec3::new(-20.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            PhaseType::TopDownShooter => (
                Vec3::new(0.0, 15.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
            ),
        };

        Phase {
            phase_type,
            camera_position,
            camera_target,
            camera_up,
            completed: false,
        }
    }

    pub fn setup_camera(&self, camera: &mut Camera, ship_position: Vec3) {
        let offset = self.camera_position;
        camera.eye = ship_position + offset;
        camera.center = ship_position;
        camera.up = self.camera_up;
    }
}

pub struct PhaseManager {
    pub phases: Vec<Phase>,
    pub current_phase_index: usize,
    transition_timer: f32,
    pub transitioning: bool,
}

impl PhaseManager {
    pub fn new() -> Self {
        let phases = vec![
            Phase::new(PhaseType::TopDown),
            Phase::new(PhaseType::Side),
            Phase::new(PhaseType::TopDownShooter),
        ];

        PhaseManager {
            phases,
            current_phase_index: 0,
            transition_timer: 0.0,
            transitioning: false,
        }
    }

    pub fn current_phase(&self) -> &Phase {
        &self.phases[self.current_phase_index]
    }

    pub fn current_phase_mut(&mut self) -> &mut Phase {
        &mut self.phases[self.current_phase_index]
    }

    pub fn next_phase(&mut self) {
        if self.current_phase_index < self.phases.len() - 1 {
            self.transitioning = true;
            self.transition_timer = 0.0;
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.transitioning {
            self.transition_timer += delta_time;
            if self.transition_timer >= 1.0 {
                self.current_phase_index += 1;
                self.transitioning = false;
                self.transition_timer = 0.0;
            }
        }
    }
}
