mod color;
mod framebuffer;
mod vertex;
mod pipeline;

use minifb::{Key, Window, WindowOptions};
use color::Color;
use framebuffer::Framebuffer;
use vertex::Vertex2D;
use pipeline::triangle;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    
    let mut window = Window::new(
        "Space Travel - Software Renderer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Unable to create window: {}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let v1 = Vertex2D::new(400.0, 100.0, Color::MAGENTA);
    let v2 = Vertex2D::new(200.0, 500.0, Color::CYAN);
    let v3 = Vertex2D::new(600.0, 500.0, Color::NEON_YELLOW);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();
        
        triangle(&v1, &v2, &v3, &mut framebuffer);

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
