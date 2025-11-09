use crate::vertex::Vertex;
use crate::uniforms::Uniforms;
use nalgebra_glm::{Vec4, Mat3};

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let mut transformed = vertex.clone();
    
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );
    
    let mvp = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix;
    transformed.transformed_position = mvp * position;
    
    let model_mat3 = Mat3::new(
        uniforms.model_matrix[0], uniforms.model_matrix[1], uniforms.model_matrix[2],
        uniforms.model_matrix[4], uniforms.model_matrix[5], uniforms.model_matrix[6],
        uniforms.model_matrix[8], uniforms.model_matrix[9], uniforms.model_matrix[10],
    );
    transformed.transformed_normal = (model_mat3 * vertex.normal).normalize();
    
    transformed
}
