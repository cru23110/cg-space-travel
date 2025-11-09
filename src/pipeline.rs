use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex2D;

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
