use pixelwin::{Color, Config, DoubleBuffer, Key, MouseButton, Window};

pub const WIDTH: usize = 64; // CHIP8 original X resolution
pub const HEIGHT: usize = 32; // CHIP8 original Y resolution


// Initialize pixelwin
fn init_pixelwin() -> Option<Window> {
    let config = Config { scale: 32, resizable: false, target_fps: Some(60) };
    match Window::new("Haakon's CHIP8 Emulator", WIDTH, HEIGHT, config) {
        Ok(w) => Some(w),
        Err(e) => { eprint!("Could not open window: {}", e); None }
    }
}

fn main() {
    // Initialize pixelwin
    let Some(mut win) = init_pixelwin() else { return };
    let mut display = [false; WIDTH * HEIGHT];

    while win.is_open() {
        // Events
        win.poll_events();
        if win.key_pressed(Key::Escape) { win.close(); }

        // Redraw the screen each frame
        win.clear(Color::BLACK);
        for y in 0..HEIGHT { for x in 0..WIDTH {
            if display[y * WIDTH + x] { win.set_pixel(x as i32, y as i32, Color::WHITE); }
        }}

        win.present();
    }   
}