pub mod chip8;
pub mod dir;
pub mod graphics;
pub mod input;

use chip8::Chip8;
use graphics::Display;

const SCREEN_X: u32 = 1320;
const SCREEN_Y: u32 = 680;
const GAMES_DIR: &str = "./GAMES";

pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let selected_game = dir::Navigator::select_game(GAMES_DIR);
    
    let mut c8: Chip8 = Chip8::new();
    let input_handler = input::Handler::new();
    c8.load_font();
    c8.load_rom(selected_game.to_string());

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rusty CHIP-8", 
        SCREEN_X,
        SCREEN_Y)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .accelerated()
        // .present_vsync()
        .build()
        .unwrap();

    Display::clear(&mut canvas);
    let mut num_cycles = 0;
    loop {
        // let frame = std::time::Duration::from_micros(1000);
        // let now = std::time::Instant::now();
        // std::thread::sleep(frame);
        c8.cycle(&input_handler, &sdl_context);
        println!("Cycles: {}", num_cycles);
        // let mut s=String::new();
        // std::io::stdin().read_line(&mut s).ok();
        c8.print_registers();
        if input_handler.set_chip8_keys(&mut c8, &sdl_context) {
            break;
        }
        // input_handler.print_chip8_keys(&c8);
        Display::render_gfx(&mut c8, &mut canvas);
        c8.count_dt();
        num_cycles += 1;        
        // ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 30));
        // ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
