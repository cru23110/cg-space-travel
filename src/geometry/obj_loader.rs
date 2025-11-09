use crate::vertex::Vertex;
use crate::color::Color;
use nalgebra_glm::{Vec2, Vec3};
use std::fs;
use std::io::{self, BufRead};

pub struct Mesh {
    pub vertices: Vec<Vertex>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>) -> Self {
        Mesh { vertices }
    }
}

pub fn load_obj(file_path: &str) -> Result<Mesh, String> {
    let file = fs::File::open(file_path)
        .map_err(|e| format!("Failed to open file {}: {}", file_path, e))?;
    
    let reader = io::BufReader::new(file);
    
    let mut positions: Vec<Vec3> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();
    let mut tex_coords: Vec<Vec2> = Vec::new();
    let mut vertices: Vec<Vertex> = Vec::new();
    
    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
        let line = line.trim();
        
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        match parts[0] {
            "v" => {
                if parts.len() >= 4 {
                    let x: f32 = parts[1].parse().unwrap_or(0.0);
                    let y: f32 = parts[2].parse().unwrap_or(0.0);
                    let z: f32 = parts[3].parse().unwrap_or(0.0);
                    positions.push(Vec3::new(x, y, z));
                }
            }
            "vn" => {
                if parts.len() >= 4 {
                    let x: f32 = parts[1].parse().unwrap_or(0.0);
                    let y: f32 = parts[2].parse().unwrap_or(0.0);
                    let z: f32 = parts[3].parse().unwrap_or(0.0);
                    normals.push(Vec3::new(x, y, z));
                }
            }
            "vt" => {
                if parts.len() >= 3 {
                    let u: f32 = parts[1].parse().unwrap_or(0.0);
                    let v: f32 = parts[2].parse().unwrap_or(0.0);
                    tex_coords.push(Vec2::new(u, v));
                }
            }
            "f" => {
                let mut face_vertices: Vec<Vertex> = Vec::new();
                
                for i in 1..parts.len() {
                    let indices: Vec<&str> = parts[i].split('/').collect();
                    
                    let pos_idx: usize = indices[0].parse::<usize>().unwrap_or(1) - 1;
                    let tex_idx: Option<usize> = if indices.len() > 1 && !indices[1].is_empty() {
                        indices[1].parse::<usize>().ok().map(|i| i - 1)
                    } else {
                        None
                    };
                    let norm_idx: Option<usize> = if indices.len() > 2 {
                        indices[2].parse::<usize>().ok().map(|i| i - 1)
                    } else {
                        None
                    };
                    
                    let position = if pos_idx < positions.len() {
                        positions[pos_idx]
                    } else {
                        Vec3::new(0.0, 0.0, 0.0)
                    };
                    
                    let normal = if let Some(idx) = norm_idx {
                        if idx < normals.len() {
                            normals[idx]
                        } else {
                            Vec3::new(0.0, 0.0, 1.0)
                        }
                    } else {
                        Vec3::new(0.0, 0.0, 1.0)
                    };
                    
                    let mut vertex = Vertex::new(position, normal, Color::WHITE);
                    
                    if let Some(idx) = tex_idx {
                        if idx < tex_coords.len() {
                            vertex.tex_coords = tex_coords[idx];
                        }
                    }
                    
                    face_vertices.push(vertex);
                }
                
                // Triangulate face (support for quads and polygons)
                for i in 1..(face_vertices.len() - 1) {
                    vertices.push(face_vertices[0]);
                    vertices.push(face_vertices[i]);
                    vertices.push(face_vertices[i + 1]);
                }
            }
            _ => {}
        }
    }
    
    Ok(Mesh::new(vertices))
}
