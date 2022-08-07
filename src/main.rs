pub mod chip8;
pub mod decoder;

use chip8::Chip8;
pub use crate::decoder::Decoder;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut _c8: Chip8 = Chip8::new();
    _c8.load_font();
    _c8.load_rom("/home/hoswoo/Desktop/Programming/Rust/rusty-chip8/GAMES/games/TEST_OP".to_string());

    let mut _num_cycles = 0;

    'running: loop {
        _c8.cycle();
        println!("Cycles: {}", _num_cycles);
        let mut s=String::new();
        std::io::stdin().read_line(&mut s).ok();
        _c8.print_registers();
        _c8.print_screen();
        _num_cycles += 1;
    }
}
