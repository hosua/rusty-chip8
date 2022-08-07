pub mod chip8;
pub mod decoder;
pub mod graphics;

use chip8::Chip8;
use graphics::Display;
pub use crate::chip8::DISP_X;
pub use crate::chip8::DISP_Y;
pub use crate::chip8::PIXEL_SIZE;

use sdl2::pixels;

pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut c8: Chip8 = Chip8::new();
    c8.load_font();
    c8.load_rom("/home/hoswoo/Desktop/Programming/Rust/rusty-chip8/GAMES/games/TEST_OP".to_string());

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Rusty CHIP-8", 
        DISP_X as u32 * PIXEL_SIZE as u32, 
        DISP_Y as u32 * PIXEL_SIZE as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    Display::clear(&mut canvas);
    // canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    // canvas.clear();
    // canvas.present();

    let mut num_cycles = 0;
    'running: loop {
        c8.cycle();
        println!("Cycles: {}", num_cycles);
        let mut s=String::new();
        std::io::stdin().read_line(&mut s).ok();
        c8.print_registers();
        c8.print_screen();
        num_cycles += 1;
    }
}
