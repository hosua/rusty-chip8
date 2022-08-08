use std::collections::HashMap;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;

use crate::chip8::Chip8;
// Provides a clean way of constructing a hashmap via a macro
// https://stackoverflow.com/questions/28392008/is-there-a-more-concise-or-declarative-way-to-initialize-a-hashmap
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key, $val); )*
            map
    }}
}
// All keyboard input logic is handled in here
pub struct Handler {
    pub key_map: HashMap<Keycode, i32>,
}

impl Handler {
    pub fn new() -> Self{
        // using hashmap macro
        // <K = Keycode, V = CHIP-8 key index>
        let key_map = hashmap![
            Keycode::Num1 => 0x1, Keycode::Num2 => 0x2, Keycode::Num3 => 0x3, Keycode::Num4 => 0xC,
            Keycode::Q => 0x4, Keycode::W => 0x5, Keycode::E => 0x6, Keycode::R => 0xD,
            Keycode::A => 0x7, Keycode::S => 0x8, Keycode::D => 0x9, Keycode::F => 0xE,
            Keycode::Z => 0xA, Keycode::X => 0x0, Keycode::C => 0xB, Keycode::V => 0xF
        ];
        Self { key_map }
    } 
    // TODO: This shit is horrible, find a better way to write this.
    // Returns true if the chip8 should exit
    pub fn set_chip8_keys(self: &Self, chip8: &mut Chip8, sdl_context: &sdl2::Sdl) -> bool {
        let mut event_pump = sdl_context.event_pump().unwrap();
        // I want to do something like this, but how???
        // for (key, val) in &self.key_map {
        //     let key_idx = *val as usize;
        // }
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => 
                    return true,
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => 
                    chip8.keys[self.key_map[&Keycode::Num1] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } =>
                    chip8.keys[self.key_map[&Keycode::Num2] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } =>
                    chip8.keys[self.key_map[&Keycode::Num3] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } =>
                    chip8.keys[self.key_map[&Keycode::Num4] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::Q), .. } =>
                    chip8.keys[self.key_map[&Keycode::Q] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::W), .. } =>
                    chip8.keys[self.key_map[&Keycode::W] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::E), .. } =>
                    chip8.keys[self.key_map[&Keycode::E] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::R), .. } =>
                    chip8.keys[self.key_map[&Keycode::R] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::A), .. } =>
                    chip8.keys[self.key_map[&Keycode::A] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::S), .. } =>
                    chip8.keys[self.key_map[&Keycode::S] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::D), .. } =>
                    chip8.keys[self.key_map[&Keycode::D] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::F), .. } =>
                    chip8.keys[self.key_map[&Keycode::F] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::Z), .. } =>
                    chip8.keys[self.key_map[&Keycode::Z] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::X), .. } =>
                    chip8.keys[self.key_map[&Keycode::X] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::C), .. } =>
                    chip8.keys[self.key_map[&Keycode::C] as usize] = true,
                Event::KeyDown { keycode: Some(Keycode::V), .. } =>
                    chip8.keys[self.key_map[&Keycode::V] as usize] = true,

                Event::KeyUp { keycode: Some(Keycode::Num1), .. } => 
                    chip8.keys[self.key_map[&Keycode::Num1] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::Num2), .. } =>
                    chip8.keys[self.key_map[&Keycode::Num2] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::Num3), .. } =>
                    chip8.keys[self.key_map[&Keycode::Num3] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::Num4), .. } =>
                    chip8.keys[self.key_map[&Keycode::Num4] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::Q), .. } =>
                    chip8.keys[self.key_map[&Keycode::Q] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::W), .. } =>
                    chip8.keys[self.key_map[&Keycode::W] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::E), .. } =>
                    chip8.keys[self.key_map[&Keycode::E] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::R), .. } =>
                    chip8.keys[self.key_map[&Keycode::R] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::A), .. } =>
                    chip8.keys[self.key_map[&Keycode::A] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::S), .. } =>
                    chip8.keys[self.key_map[&Keycode::S] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::D), .. } =>
                    chip8.keys[self.key_map[&Keycode::D] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::F), .. } =>
                    chip8.keys[self.key_map[&Keycode::F] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::Z), .. } =>
                    chip8.keys[self.key_map[&Keycode::Z] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::X), .. } =>
                    chip8.keys[self.key_map[&Keycode::X] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::C), .. } =>
                    chip8.keys[self.key_map[&Keycode::C] as usize] = false,
                Event::KeyUp { keycode: Some(Keycode::V), .. } =>
                    chip8.keys[self.key_map[&Keycode::V] as usize] = false,
                
                _ => {}
            }
        }
        return false;
    }

    pub fn wait_for_key(self: &Self, chip8: &mut Chip8, sdl_context: &sdl2::Sdl) -> u8 {
        println!("WAIT FOR KEY");
        use sdl2::event::Event;
        let mut event_pump = sdl_context.event_pump().unwrap();
        loop {
            let event = event_pump.wait_event();
            // TODO: This is stupid and there should be a better way to do this
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return 0x69,

                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => return 0x1,
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => return 0x2,
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => return 0x3,
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => return 0xC,

                Event::KeyDown { keycode: Some(Keycode::Q), .. } => return 0x4,
                Event::KeyDown { keycode: Some(Keycode::W), .. } => return 0x5,
                Event::KeyDown { keycode: Some(Keycode::E), .. } => return 0x6,
                Event::KeyDown { keycode: Some(Keycode::R), .. } => return 0xD,

                Event::KeyDown { keycode: Some(Keycode::A), .. } => return 0x7,
                Event::KeyDown { keycode: Some(Keycode::S), .. } => return 0x8,
                Event::KeyDown { keycode: Some(Keycode::D), .. } => return 0x9,
                Event::KeyDown { keycode: Some(Keycode::F), .. } => return 0xE,

                Event::KeyDown { keycode: Some(Keycode::Z), .. } => return 0xA,
                Event::KeyDown { keycode: Some(Keycode::X), .. } => return 0x0,
                Event::KeyDown { keycode: Some(Keycode::C), .. } => return 0xB,
                Event::KeyDown { keycode: Some(Keycode::V), .. } => return 0xF,
                _ => {} 
            }
        }
    }
    pub fn print_chip8_keys(self: &Self, chip8: &Chip8){
        let keys = chip8.keys; 
        // 1 2 3 C
        for i in 0x1..0x3+1 {
            print!("{} ", keys[i] as u8);
        }
        println!("{}", keys[0xC] as u8);
        // 4 5 6 D
        for i in 0x4..0x6+1 {
            print!("{} ", keys[i] as u8);
        }
        println!("{}", keys[0xD] as u8);
        // 7 8 9 E
        for i in 0x7..0x9+1 {
            print!("{} ", keys[i] as u8);
        }
        println!("{}", keys[0xE] as u8);
        // A 0 B F
        print!("{} ", keys[0xA] as u8);
        print!("{} ", keys[0x0] as u8);
        print!("{} ", keys[0xB] as u8);
        println!("{} ", keys[0xF] as u8);
    }

}

