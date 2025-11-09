use crate::vertex::Vertex;
use crate::color::Color;
use crate::geometry::Mesh;
use nalgebra_glm::Vec3;
use std::f32::consts::PI;

pub fn create_sphere(radius: f32, segments: u32, rings: u32) -> Mesh {
    let mut vertices = Vec::new();

    for ring in 0..=rings {
        let theta = ring as f32 * PI / rings as f32;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        for segment in 0..=segments {
            let phi = segment as f32 * 2.0 * PI / segments as f32;
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            let x = sin_theta * cos_phi;
            let y = cos_theta;
            let z = sin_theta * sin_phi;

            let position = Vec3::new(x * radius, y * radius, z * radius);
            let normal = Vec3::new(x, y, z).normalize();

            vertices.push(Vertex::new(position, normal, Color::WHITE));
        }
    }

    let mut triangles = Vec::new();

    for ring in 0..rings {
        for segment in 0..segments {
            let current = (ring * (segments + 1) + segment) as usize;
            let next = current + (segments + 1) as usize;

            triangles.push(vertices[current]);
            triangles.push(vertices[next]);
            triangles.push(vertices[current + 1]);

            triangles.push(vertices[current + 1]);
            triangles.push(vertices[next]);
            triangles.push(vertices[next + 1]);
        }
    }

    Mesh::new(triangles)
}
