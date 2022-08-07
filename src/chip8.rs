const MEM_SIZE: usize = 4096;
const DISP_X: usize = 64;
const DISP_Y: usize = 32;
pub const NUM_VREGS: usize = 16;
const NUM_KEYS: usize = 16;
const PX: &'static str = "\u{2588}\u{2588}";


use crate::decoder::Decoder;

pub struct Chip8 {
    mem: [u8; MEM_SIZE],
    // gfx can technically be a boolean array but I prefer using u8 
    // so that I can cleanly XOR its values
    gfx: [u8; DISP_X * DISP_Y],
    keys: [bool; NUM_KEYS],
    draw_flag: bool,

    v: [u8; NUM_VREGS], 
    i: u16,
    pc: usize,
    dt: u8,
    st: u8,
    opcode: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        let mem: [u8; MEM_SIZE] = [0; MEM_SIZE];
        let gfx: [u8; DISP_X * DISP_Y] = [0; DISP_X * DISP_Y];
        let keys: [bool; NUM_KEYS] = [false; NUM_KEYS];
        let draw_flag: bool = false;

        // TODO: Add stack and clock
        let v: [u8; NUM_VREGS] = [0; NUM_VREGS];
        let i: u16 = 0x0;
        let pc: usize = 0x200;
        let dt: u8 = 0x0;
        let st: u8 = 0x0;
        let opcode: u16 = 0x0;
        Self { mem, gfx, keys, draw_flag, v, i, pc, dt, st, opcode }
    }
    // Load all font data to chip8 memory
    pub fn load_font(self: &mut Self){
        let textfont: [u8; 80] = [
            // 1,    2, 	3, 	  4, 	5 bytes
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        self.mem[0..80].clone_from_slice(&textfont);
    }

    // Load rom into memory starting at address 0x200
    pub fn load_rom(self: &mut Self, rom_path: String){
        println!("Loading game: {}", rom_path);
        let mut _file = std::fs::File::open(&rom_path).expect("No file found");        
        // get file metadata
        let metadata = std::fs::metadata(&rom_path).unwrap();
        // copy raw file data to memory (fs::read conveniently returns a u8 vector)
        let mut _rom_data = std::fs::read(&rom_path).unwrap();

        self.mem[0x200..0x200+_rom_data.len()]
            .clone_from_slice(&_rom_data);

        let _filesize = metadata.len();
        println!("{} bits loaded into memory", _filesize);
        // close file
        drop(_file);

    }

    pub fn cycle(self: &mut Self){
        self.fetch();
        self.execute();
        self.pc += 2;
    }

    pub fn fetch(self: &mut Self){
        self.opcode = ((self.mem[self.pc] as u16) << 8) | (self.mem[self.pc + 1]) as u16;
        println!("{:#X}: ({:#X})", self.pc, self.opcode);
    }

    pub fn execute(self: &mut Self){
        // Decode variables from opcode instruction
        let _x = Decoder::x(self.opcode);
        let _y = Decoder::y(self.opcode);
        let _kk = Decoder::kk(self.opcode);
        let _nnn = Decoder::nnn(self.opcode);
        let _n = Decoder::n(self.opcode);

        let mut _opstr = "";
        // Execute the instruction
        match self.opcode & 0xF000 {
            // CLS - Clear screen
            0x00E0 => {
                self.gfx[0..MEM_SIZE].fill(0);
                self.draw_flag = true;
            },
            // JP: Jump to nnn
            0x1000 => {
                _opstr = "JP";
                self.pc = _nnn as usize;
                self.pc -= 2;
                println!("{} {:#X}", _opstr, _nnn)
            },

            // 6xkk - Set Vx to kk
            0x6000 => {
                _opstr = "LD";
                self.v[_x] = _kk;
                println!("{} Vx = kk({:X})", _opstr, _kk);
            },
            // LD - Set I to nnn
            0xA000 => {
                _opstr = "LD";
                self.i = _nnn;
                println!("I = nnn({:#3X})", _nnn);
            },
            // Dxyn - Draw
            0xD000 => {
                self.v[0xF] = 0x0;
                let mut _px: u8 = 0x0;
                // Reduce if overflow
                if self.v[_x] > DISP_X as u8 {
                    self.v[_x] %= DISP_X as u8; 
                }
                if self.v[_y] > DISP_Y as u8 {
                    self.v[_y] %= DISP_Y as u8; 
                }

                for _dy in 0.._n as usize {
                    _px = self.mem[self.i as usize + _dy];
                    for _dx in 0..8 as usize {
                        if _px & (0x80 >> _dx) != 0 {
                            // If a pixel is drawn to an already drawn pixel, it is unset
                            if self.gfx[(self.v[_x] as usize + _dx + ((self.v[_y] as usize + _dy) * DISP_X))] != 0{
                                // Indicate that pixel was unset
                                self.v[0xF] = 1;
                            }
                            self.gfx[self.v[_x] as usize + _dx + ((self.v[_y] as usize + _dy) * DISP_X)] ^= 1;
                        }
                    }
                }

                self.draw_flag = true;
            },
            _ => println!("Invalid opcode {:#X}", self.opcode),
        }
    }
    
    pub fn print_registers(self: &Self){
        println!("------------------------");
        println!("V REGISTERS:");
        for _i in 0..NUM_VREGS {
            print!("{:#X}\t", _i);
        }
        println!();

        for _i in 0..NUM_VREGS {
            print!("{:#X}\t", self.v[_i]);
        }
        println!();

        println!("I: {:#X}", self.i);
        println!("DT: {:#X}", self.dt);
        println!("ST: {:#X}", self.st);
    }
    // TODO: This is temporarily acting as a display, we eventually
    // need to implement this in sdl2
    pub fn print_screen(self: &mut Self){
        if self.draw_flag {
            self.draw_flag = false;
            for _i in 0..DISP_X*DISP_Y {
                if self.gfx[_i] != 0 {
                    print!("{}", PX);
                } else {
                    print!("  ");
                }
                if (_i + 1) % DISP_X == 0 {
                    println!();
                }
            }
        }
    }
}
