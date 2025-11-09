pub mod obj_loader;

use crate::vertex::Vertex;
use crate::color::Color;
use nalgebra_glm::Vec3;

pub use obj_loader::{Mesh, load_obj};

pub fn create_cube() -> Vec<Vertex> {
    let vertices = vec![
        Vertex::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(0.0, 0.0, -1.0), Color::MAGENTA),
        Vertex::new(Vec3::new(1.0, -1.0, -1.0), Vec3::new(0.0, 0.0, -1.0), Color::MAGENTA),
        Vertex::new(Vec3::new(1.0, 1.0, -1.0), Vec3::new(0.0, 0.0, -1.0), Color::MAGENTA),
        Vertex::new(Vec3::new(1.0, 1.0, -1.0), Vec3::new(0.0, 0.0, -1.0), Color::MAGENTA),
        Vertex::new(Vec3::new(-1.0, 1.0, -1.0), Vec3::new(0.0, 0.0, -1.0), Color::MAGENTA),
        Vertex::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(0.0, 0.0, -1.0), Color::MAGENTA),

        Vertex::new(Vec3::new(-1.0, -1.0, 1.0), Vec3::new(0.0, 0.0, 1.0), Color::CYAN),
        Vertex::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 0.0, 1.0), Color::CYAN),
        Vertex::new(Vec3::new(1.0, -1.0, 1.0), Vec3::new(0.0, 0.0, 1.0), Color::CYAN),
        Vertex::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 0.0, 1.0), Color::CYAN),
        Vertex::new(Vec3::new(-1.0, -1.0, 1.0), Vec3::new(0.0, 0.0, 1.0), Color::CYAN),
        Vertex::new(Vec3::new(-1.0, 1.0, 1.0), Vec3::new(0.0, 0.0, 1.0), Color::CYAN),

        Vertex::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(-1.0, 0.0, 0.0), Color::NEON_YELLOW),
        Vertex::new(Vec3::new(-1.0, 1.0, 1.0), Vec3::new(-1.0, 0.0, 0.0), Color::NEON_YELLOW),
        Vertex::new(Vec3::new(-1.0, -1.0, 1.0), Vec3::new(-1.0, 0.0, 0.0), Color::NEON_YELLOW),
        Vertex::new(Vec3::new(-1.0, 1.0, 1.0), Vec3::new(-1.0, 0.0, 0.0), Color::NEON_YELLOW),
        Vertex::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(-1.0, 0.0, 0.0), Color::NEON_YELLOW),
        Vertex::new(Vec3::new(-1.0, 1.0, -1.0), Vec3::new(-1.0, 0.0, 0.0), Color::NEON_YELLOW),

        Vertex::new(Vec3::new(1.0, -1.0, -1.0), Vec3::new(1.0, 0.0, 0.0), Color::PURPLE),
        Vertex::new(Vec3::new(1.0, -1.0, 1.0), Vec3::new(1.0, 0.0, 0.0), Color::PURPLE),
        Vertex::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 0.0, 0.0), Color::PURPLE),
        Vertex::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 0.0, 0.0), Color::PURPLE),
        Vertex::new(Vec3::new(1.0, 1.0, -1.0), Vec3::new(1.0, 0.0, 0.0), Color::PURPLE),
        Vertex::new(Vec3::new(1.0, -1.0, -1.0), Vec3::new(1.0, 0.0, 0.0), Color::PURPLE),

        Vertex::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(0.0, -1.0, 0.0), Color::RED),
        Vertex::new(Vec3::new(1.0, -1.0, 1.0), Vec3::new(0.0, -1.0, 0.0), Color::RED),
        Vertex::new(Vec3::new(1.0, -1.0, -1.0), Vec3::new(0.0, -1.0, 0.0), Color::RED),
        Vertex::new(Vec3::new(1.0, -1.0, 1.0), Vec3::new(0.0, -1.0, 0.0), Color::RED),
        Vertex::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(0.0, -1.0, 0.0), Color::RED),
        Vertex::new(Vec3::new(-1.0, -1.0, 1.0), Vec3::new(0.0, -1.0, 0.0), Color::RED),

        Vertex::new(Vec3::new(-1.0, 1.0, -1.0), Vec3::new(0.0, 1.0, 0.0), Color::GREEN),
        Vertex::new(Vec3::new(1.0, 1.0, -1.0), Vec3::new(0.0, 1.0, 0.0), Color::GREEN),
        Vertex::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 0.0), Color::GREEN),
        Vertex::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 0.0), Color::GREEN),
        Vertex::new(Vec3::new(-1.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 0.0), Color::GREEN),
        Vertex::new(Vec3::new(-1.0, 1.0, -1.0), Vec3::new(0.0, 1.0, 0.0), Color::GREEN),
    ];

    vertices
}
