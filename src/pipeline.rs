use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::vertex::{Vertex2D, Vertex};
use crate::fragment::Fragment;
use crate::uniforms::Uniforms;
use crate::shaders::vertex_shader::vertex_shader;
use crate::shaders::fragment_shader::fragment_shader;

fn edge_function(a: &Vertex2D, b: &Vertex2D, c: &Vertex2D) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}

pub fn triangle(v1: &Vertex2D, v2: &Vertex2D, v3: &Vertex2D, framebuffer: &mut Framebuffer) {
    let min_x = v1.x.min(v2.x).min(v3.x).max(0.0) as usize;
    let min_y = v1.y.min(v2.y).min(v3.y).max(0.0) as usize;
    let max_x = v1.x.max(v2.x).max(v3.x).min(framebuffer.width as f32 - 1.0) as usize;
    let max_y = v1.y.max(v2.y).max(v3.y).min(framebuffer.height as f32 - 1.0) as usize;

    let area = edge_function(v1, v2, v3);
    
    if area.abs() < 0.0001 {
        return;
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Vertex2D::new(x as f32 + 0.5, y as f32 + 0.5, Color::BLACK);

            let w1 = edge_function(v2, v3, &p);
            let w2 = edge_function(v3, v1, &p);
            let w3 = edge_function(v1, v2, &p);

            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                let w1 = w1 / area;
                let w2 = w2 / area;
                let w3 = w3 / area;

                let r = (v1.color.r as f32 * w1 + v2.color.r as f32 * w2 + v3.color.r as f32 * w3) as u8;
                let g = (v1.color.g as f32 * w1 + v2.color.g as f32 * w2 + v3.color.g as f32 * w3) as u8;
                let b = (v1.color.b as f32 * w1 + v2.color.b as f32 * w2 + v3.color.b as f32 * w3) as u8;

                let color = Color::new(r, g, b);
                framebuffer.point(x, y, &color);
            }
        }
    }
}

pub fn triangle_3d(v1: &Vertex, v2: &Vertex, v3: &Vertex, uniforms: &Uniforms, framebuffer: &mut Framebuffer) {
    let vt1 = vertex_shader(v1, uniforms);
    let vt2 = vertex_shader(v2, uniforms);
    let vt3 = vertex_shader(v3, uniforms);

    let ndc1 = vt1.transformed_position / vt1.transformed_position.w;
    let ndc2 = vt2.transformed_position / vt2.transformed_position.w;
    let ndc3 = vt3.transformed_position / vt3.transformed_position.w;

    let screen1 = uniforms.viewport_matrix * ndc1;
    let screen2 = uniforms.viewport_matrix * ndc2;
    let screen3 = uniforms.viewport_matrix * ndc3;

    let v1_screen = Vertex2D::new(screen1.x, screen1.y, vt1.color);
    let v2_screen = Vertex2D::new(screen2.x, screen2.y, vt2.color);
    let v3_screen = Vertex2D::new(screen3.x, screen3.y, vt3.color);

    let min_x = v1_screen.x.min(v2_screen.x).min(v3_screen.x).max(0.0) as usize;
    let min_y = v1_screen.y.min(v2_screen.y).min(v3_screen.y).max(0.0) as usize;
    let max_x = v1_screen.x.max(v2_screen.x).max(v3_screen.x).min(framebuffer.width as f32 - 1.0) as usize;
    let max_y = v1_screen.y.max(v2_screen.y).max(v3_screen.y).min(framebuffer.height as f32 - 1.0) as usize;

    let area = edge_function(&v1_screen, &v2_screen, &v3_screen);

    if area.abs() < 0.0001 {
        return;
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Vertex2D::new(x as f32 + 0.5, y as f32 + 0.5, Color::BLACK);

            let w1 = edge_function(&v2_screen, &v3_screen, &p);
            let w2 = edge_function(&v3_screen, &v1_screen, &p);
            let w3 = edge_function(&v1_screen, &v2_screen, &p);

            // Check if point is inside triangle (handle both CW and CCW winding)
            let inside = if area > 0.0 {
                w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0
            } else {
                w1 <= 0.0 && w2 <= 0.0 && w3 <= 0.0
            };

            if inside {
                let w1 = w1 / area;
                let w2 = w2 / area;
                let w3 = w3 / area;

                let depth = ndc1.z * w1 + ndc2.z * w2 + ndc3.z * w3;

                let normal = (vt1.transformed_normal * w1 + vt2.transformed_normal * w2 + vt3.transformed_normal * w3).normalize();

                let r = (vt1.color.r as f32 * w1 + vt2.color.r as f32 * w2 + vt3.color.r as f32 * w3) as u8;
                let g = (vt1.color.g as f32 * w1 + vt2.color.g as f32 * w2 + vt3.color.g as f32 * w3) as u8;
                let b = (vt1.color.b as f32 * w1 + vt2.color.b as f32 * w2 + vt3.color.b as f32 * w3) as u8;

                let color = Color::new(r, g, b);

                let mut fragment = Fragment::new(x, y, depth);
                fragment.color = color;
                fragment.normal = normal;

                let shaded_color = fragment_shader(&fragment, uniforms);
                framebuffer.point_with_depth(x, y, depth, &shaded_color);
            }
        }
    }
}
