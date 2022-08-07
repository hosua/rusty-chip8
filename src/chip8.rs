const MEM_SIZE: usize = 4096;
const MSB_POS: usize = 7;
const DISP_X: usize = 64;
const DISP_Y: usize = 32;
pub const NUM_VREGS: usize = 16;
const NUM_KEYS: usize = 16;
const PX: &'static str = "\u{2588}\u{2588}";

use crate::decoder::Decoder;
use std::num::Wrapping;
use std::vec::Vec;
use rand::Rng;

pub struct Chip8 {
    mem: [u8; MEM_SIZE],
    // gfx can technically be a boolean array but I prefer using u8 
    // so that I can cleanly XOR its values
    gfx: [u8; DISP_X * DISP_Y],
    stack: Vec<u16>,
    keys: [bool; NUM_KEYS],
    draw_flag: bool,

    // Registers
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
        let stack: Vec<u16> = Vec::new();

        // TODO: Add stack and clock
        let v: [u8; NUM_VREGS] = [0; NUM_VREGS];
        let i: u16 = 0x0;
        let pc: usize = 0x200;
        let dt: u8 = 0x0;
        let st: u8 = 0x0;
        let opcode: u16 = 0x0;
        Self { mem, gfx, keys, draw_flag, stack, 
               v, i, pc, dt, st, 
               opcode }
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
    // Execute a cpu cycle
    pub fn cycle(self: &mut Self){
        self.fetch();
        self.execute();
        self.pc += 2;
    }
    // Fetches opcode from data addressed by the program counter
    fn fetch(self: &mut Self){
        self.opcode = ((self.mem[self.pc] as u16) << 8) | (self.mem[self.pc + 1]) as u16;
        println!("{:#X}: ({:#X})", self.pc, self.opcode);
    }
    
    // Decodes and executes opcode instructions
    fn execute(self: &mut Self){
        // Decode variables from opcode instruction
        let _x = Decoder::x(self.opcode);
        let _y = Decoder::y(self.opcode);
        let _kk = Decoder::kk(self.opcode);
        let _nnn = Decoder::nnn(self.opcode);
        let _n = Decoder::n(self.opcode);

        let mut _opstr = "";
        // Execute the instruction
        match self.opcode & 0xF000 {
            0x0000 => {
                match self.opcode & 0x000F {
                    // 00E0: CLS - Clear screen
                    0x0000 => {
                        self.gfx[0..MEM_SIZE].fill(0);
                        self.draw_flag = true;
                    }
                    // 00EE: RET - Return from subroutine
                    0x000E => {
                        self.pc = self.stack.last().unwrap().clone() as usize;
                        self.stack.pop();
                        self.draw_flag = true;
                    }
                    _ => {
                        eprintln!("Invalid 0x0000 opcode ({:#X})", self.opcode);
                    }
                }
            }
            // 1nnn: JP - Jump to nnn
            0x1000 => {
                _opstr = "JP";
                self.pc = _nnn as usize;
                self.pc -= 2;
                println!("{} {:#X}", _opstr, _nnn)
            }
            
            // 2nnn: CALL addr
            0x2000 => {
                _opstr = "CALL";
                // Call subroutine at nnn
                self.stack.push(self.pc as u16);
                // Jump to nnn
                self.pc = _nnn as usize;
                self.pc -= 2;
                println!("{} {:#X}", _opstr, _nnn)
            }
            // 3xkk: SE - Skip instruction if if Vx == kk
            0x3000 => {
                _opstr = "SE"; 
                if self.v[_x] == _kk {
                    self.pc += 2;
                    println!("{} Vx == kk", _opstr);
                    println!("SKIPPING INSTRUCTION");
                } else {
                    println!("{} Vx != kk", _opstr);
                    println!("NOT SKIPPING INSTRUCTION");
                }
            }
            
            // 4xkk: SNE - Skip if next instruction if Vx != kk
            0x4000 => {
                _opstr = "SNE"; 
                if self.v[_x] != _kk {
                    self.pc += 2;
                    println!("{} Vx != kk", _opstr);
                    println!("SKIPPING INSTRUCTION");
                } else {
                    println!("{} Vx == kk", _opstr);
                    println!("NOT SKIPPING INSTRUCTION");
                }
            }

            // 5xy0: SE Skip if Vx == Vy
            0x5000 => {
                _opstr = "SE"; 
                if self.v[_x] == self.v[_y] {
                    self.pc += 2;
                    println!("{} v[{:#X}] == v[{:#X}]", _opstr, _x, _y);
                    println!("SKIPPING INSTRUCTION");
                } else {
                    println!("{} v[{:#X}] != v[{:#X}]", _opstr, _x, _y);
                    println!("NOT SKIPPING INSTRUCTION");
                }
            }

            // 6xkk: LD - Set Vx to kk
            0x6000 => {
                _opstr = "LD";
                self.v[_x] = _kk;
                println!("{} Vx = kk({:#X})", _opstr, _kk);
            }
            
            // 7xkk: ADD - Add kk to Vx
            0x7000 => {
                _opstr = "ADD";
                let Wrapping(_vxkk) = Wrapping(self.v[_x]) + Wrapping(_kk);
                self.v[_x] = _vxkk;
                println!("{} Vx += kk({:#X})", _opstr, _kk);
            }
            0x8000 => {
                match self.opcode & 0x000F {
                    // 8xy0: LD - Set Vx = Vy
                    0x0000 => {
                        _opstr = "LD";
                        self.v[_x] = self.v[_y];
                        println!("{} Vx = Vy({:#X})", _opstr, self.v[_y]);
                    }
                    // 8xy1: OR Vx |= Vy
                    0x0001 => {
                        _opstr = "OR";
                        self.v[_x] |= self.v[_y];
                        println!("{} Vx |= Vy({:#X})", _opstr, self.v[_y]);
                    }
                    // 8xy2: AND Vx &= Vy
                    0x0002 => {
                        // TODO
                        _opstr = "AND";
                        self.v[_x] &= self.v[_y];
                        println!("{} Vx &= Vy({:#X})", _opstr, self.v[_y]);
                    }
                    // 8xy3: XOR Vx ^= Vy
                    0x0003 => {
                        // TODO
                        _opstr = "XOR";
                        self.v[_x] ^= self.v[_y];
                        println!("{} Vx &= Vy({:#X})", _opstr, self.v[_y]);
                    }
                    // 8xy4: ADD - Add Vy to Vx
                    0x0004 => {
                    _opstr = "ADD";
                        let Wrapping(_vxy) = Wrapping(self.v[_y]) + Wrapping(self.v[_x]);
                        self.v[_x] = _vxy;
                        if self.v[_y] > self.v[_x] {
                            // Carry
                            self.v[0xF] = 1;
                        } else {
                            // No carry
                            self.v[0xF] = 0;
                        }
                        println!("{} Vx += Vy({:#X})", _opstr, self.v[_y]);
                    }
                    // 8xy5: SUB Vx -= Vy
                    0x0005 => {
                        // TODO
                        _opstr = "SUB";
                        let Wrapping(_vxy) = Wrapping(self.v[_x]) - Wrapping(self.v[_y]);
                        self.v[_x] = _vxy;
                        println!("{} Vx -= Vy({:#X})", _opstr, self.v[_y]);
                    }
                    // 8xy6: SHR - Shift Vx right 1 
                    0x0006 => {
                        _opstr = "SHR";
                        // Set VF if lsb = 1
                        self.v[0xF] = self.v[_x] & 1;
                        self.v[_x] >>= 1;
                        println!("{} Vx >>= 1", _opstr);
                    }
                    // 8xy7: SUBN Vx = Vy - Vx
                    0x0007 => {
                        _opstr = "SUBN";
                        let Wrapping(_vyx) = Wrapping(self.v[_y]) - Wrapping(self.v[_x]);
                        self.v[_x] = _vyx;
                        println!("{} Vx = Vy({:#X}) - Vx({:#X})", _opstr, self.v[_y], self.v[_x]);
                    }
                    // 8xyE: SHL - Shift Vx left 1 
                    0x000E => {
                        // Set VF if lsb = 1
                        self.v[0xF] = self.v[_x] >> MSB_POS;
                        self.v[_x] <<= 1;
                        println!("{} Vx <<= 1", _opstr);
                    }
                    _ => {
                        eprintln!("Invalid 0x8000 opcode: {:#X}", self.opcode);
                    }
                }
            }
            // 9xy0: SNE - Skip next instruction if Vx != Vy
            0x9000 => {
                _opstr = "SNE";
                if self.v[_x] != self.v[_y] {
                    self.pc += 2;
                    println!("{} Vx != Vy", _opstr);
                    println!("SKIPPING INSTRUCTION");
                } else {
                    println!("{} Vx == Vy", _opstr);
                    println!("NOT SKIPPING INSTRUCTION");
                }
            }
            // Annn: LD - Set I to nnn
            0xA000 => {
                _opstr = "LD";
                self.i = _nnn;
                println!("{} I = nnn({:#3X})", _opstr, _nnn);
            }
            // Bnnn: JP - Jump to nnn + V0
            0xB000 => {
                _opstr = "JP";
                self.pc = self.v[0x0] as usize + _nnn as usize;
                self.pc -= 2;
                println!("{} nnn({:#X}) + V0({:#X})", _opstr, _nnn, self.v[0x0]);
            }
            // Cxkk - RND - Generate random number from 0-255, then & kk and store the result in Vx
            0xC000 => {
                _opstr = "RND";
                self.v[_x] = rand::thread_rng().gen_range(0..0xFF) & _kk;
                println!("{} kk({:#X}) = {:#X}", _opstr, _kk, self.v[_x]);
            }
            // Dxyn: DRW - Draw
            0xD000 => {
                _opstr = "DRW";
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
                println!("{} Vx({:#X}) Vy({:#X})", _opstr, self.v[_y], self.v[_x]);
                self.draw_flag = true;
            }
            // TODO
            // Ex9E: SKP - Skip next instruction if key with Vx is pressed
            // ExA1: SKNP - Skip next instruction if key with Vx is not presed
            // Fx07: LD Vx, DT - Set Vx = delay timer value.
            // Fx0A: LD Vx, K - Wait for a key press and store the value of the key in Vx.
            // Fx15: LD DT, Vx - Set delay timer = Vx.
            // Fx18: LD ST, Vx - Set sound timer = Vx.
            // Fx1E: ADD I, Vx - I += Vx
            // Fx29: LD F, Vx - Set I = location of sprite for digit Vx.
            // Fx33: LD B, Vx - Store BCD representation of Vx in memory locations I, I+1, and I+2.
            // Fx55: LD [I], Vx - Store registers V0 through Vx in memory starting at location I.
            // Fx65 - LD Vx, [I] - Read registers V0 through Vx from memory starting at location I.
            _ => eprintln!("Invalid opcode {:#X}", self.opcode),
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
