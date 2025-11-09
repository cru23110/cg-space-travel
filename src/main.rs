use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0x001a1a2e; WIDTH * HEIGHT];

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
