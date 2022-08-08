pub const PIXEL_SIZE: usize = 20;
pub const DISP_X: usize = 64;
pub const DISP_Y: usize = 32;
pub const NUM_VREGS: usize = 16;
const MEM_SIZE: usize = 4096;
const MSB_POS: usize = 7;
const NUM_KEYS: usize = 16;
const PX: &'static str = "\u{2588}\u{2588}";
use crate::input;
use std::num::Wrapping;
use std::vec::Vec;
use rand::Rng;

pub struct Chip8 {
    mem: [u8; MEM_SIZE],
    // gfx can technically be a boolean array but I prefer using u8 
    // so that I can cleanly XOR its values
    pub gfx: [u8; DISP_X * DISP_Y],
    stack: Vec<u16>,
    pub keys: [bool; NUM_KEYS],
    pub draw_flag: bool,

    // Registers
    v: [u8; NUM_VREGS], 
    i: u16,
    pc: usize,
    dt: u8,
    st: u8,
    opcode: u16,

    pub exit_flag: bool,
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

        let exit_flag = false;
        Self { mem, gfx, keys, draw_flag, stack, 
               v, i, pc, dt, st, 
               opcode,
               exit_flag }
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
    pub fn cycle(self: &mut Self, input_handler: &input::Handler, sdl_context: &sdl2::Sdl){
        self.fetch();
        self.execute(input_handler, sdl_context);
        self.pc += 2;
    }
    // Fetches opcode from data addressed by the program counter
    fn fetch(self: &mut Self){
        self.opcode = ((self.mem[self.pc] as u16) << 8) | (self.mem[self.pc + 1]) as u16;
        println!("{:#X}: ({:#X})", self.pc, self.opcode);
    }
    
    // Decodes and executes opcode instructions
    fn execute(self: &mut Self, input_handler: &input::Handler, sdl_context: &sdl2::Sdl) {
        // Decode variables from opcode 
        let x = ((self.opcode & 0x0F00) >> 8) as usize;        
        let y = ((self.opcode & 0x00F0) >> 4) as usize;        
        let kk = (self.opcode & 0x00FF) as u8;
        let nnn = (self.opcode & 0x0FFF) as u16;
        let n = (self.opcode & 0x000F) as u8;

        let mut opstr = "";
        // Decode and execute the instruction
        match self.opcode & 0xF000 {
            0x0000 => {
                match self.opcode & 0x000F {
                    // 00E0: CLS - Clear screen
                    0x0000 => {
                        opstr = "CLS";
                        self.gfx[0..DISP_X * DISP_Y].fill(0);
                        self.draw_flag = true;
                        println!("{}", opstr);
                    }
                    // 00EE: RET - Return from subroutine
                    0x000E => {
                        opstr = "RET";
                        self.pc = self.stack.last().unwrap().clone() as usize;
                        self.stack.pop();
                        self.draw_flag = true;
                        println!("{}", opstr);
                    }
                    _ => {
                        eprintln!("Invalid 0x0000 opcode ({:#6X})", self.opcode);
                    }
                }
            }
            // 1nnn: JP - Jump to nnn
            0x1000 => {
                opstr = "JP";
                self.pc = nnn as usize;
                self.pc -= 2;
                println!("{} {:#06X}", opstr, nnn)
            }
            
            // 2nnn: CALL addr
            0x2000 => {
                opstr = "CALL";
                // Call subroutine at nnn
                self.stack.push(self.pc as u16);
                // Jump to nnn
                self.pc = nnn as usize;
                self.pc -= 2;
                println!("{} {:#06X}", opstr, nnn)
            }
            // 3xkk: SE - Skip instruction if if Vx == kk
            0x3000 => {
                opstr = "SE"; 
                if self.v[x] == kk {
                    self.pc += 2;
                    println!("{} Vx == kk{:#04X}", opstr, kk);
                    println!("SKIPPING INSTRUCTION");
                } else {
                    println!("{} Vx != kk{:#04X}", opstr, kk);
                    println!("NOT SKIPPING INSTRUCTION");
                }
            }
            
            // 4xkk: SNE - Skip if next instruction if Vx != kk
            0x4000 => {
                opstr = "SNE"; 
                if self.v[x] != kk {
                    self.pc += 2;
                    println!("{} Vx != kk{:#04X}", opstr, kk);
                    println!("SKIPPING INSTRUCTION");
                } else {
                    println!("{} Vx == kk{:#04X}", opstr, kk);
                    println!("NOT SKIPPING INSTRUCTION");
                }
            }

            // 5xy0: SE Skip if Vx == Vy
            0x5000 => {
                opstr = "SE"; 
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                    println!("{} v[{:#06X}] == v[{:#06X}]", opstr, x, y);
                    println!("SKIPPING INSTRUCTION");
                } else {
                    println!("{} v[{:#06X}] != v[{:#06X}]", opstr, x, y);
                    println!("NOT SKIPPING INSTRUCTION");
                }
            }

            // 6xkk: LD - Set Vx to kk
            0x6000 => {
                opstr = "LD";
                self.v[x] = kk;
                println!("{} Vx = kk({:#04X})", opstr, kk);
            }
            
            // 7xkk: ADD - Add kk to Vx
            0x7000 => {
                opstr = "ADD";
                let Wrapping(_vxkk) = Wrapping(self.v[x]) + Wrapping(kk);
                self.v[x] = _vxkk;
                println!("{} Vx += kk({:#06X})", opstr, kk);
            }
            0x8000 => {
                match self.opcode & 0x000F {
                    // 8xy0: LD - Set Vx = Vy
                    0x0000 => {
                        opstr = "LD";
                        self.v[x] = self.v[y];
                        println!("{} Vx = Vy({:#06X})", opstr, self.v[y]);
                    }
                    // 8xy1: OR Vx |= Vy
                    0x0001 => {
                        opstr = "OR";
                        self.v[x] |= self.v[y];
                        println!("{} Vx |= Vy({:#06X})", opstr, self.v[y]);
                    }
                    // 8xy2: AND Vx &= Vy
                    0x0002 => {
                        opstr = "AND";
                        self.v[x] &= self.v[y];
                        println!("{} Vx &= Vy({:#06X})", opstr, self.v[y]);
                    }
                    // 8xy3: XOR Vx ^= Vy
                    0x0003 => {
                        opstr = "XOR";
                        self.v[x] ^= self.v[y];
                        println!("{} Vx &= Vy({:#06X})", opstr, self.v[y]);
                    }
                    // 8xy4: ADD - Add Vy to Vx
                    0x0004 => {
                    opstr = "ADD";
                        let Wrapping(_vxy) = Wrapping(self.v[y]) + Wrapping(self.v[x]);
                        self.v[x] = _vxy;
                        if self.v[y] > self.v[x] {
                            // Carry
                            self.v[0xF] = 1;
                        } else {
                            // No carry
                            self.v[0xF] = 0;
                        }
                        println!("{} Vx += Vy({:#06X})", opstr, self.v[y]);
                    }
                    // 8xy5: SUB Vx -= Vy
                    0x0005 => {
                        opstr = "SUB";
                        let Wrapping(_vxy) = Wrapping(self.v[x]) - Wrapping(self.v[y]);
                        self.v[x] = _vxy;
                        println!("{} Vx -= Vy({:#06X})", opstr, self.v[y]);
                    }
                    // 8xy6: SHR - Shift Vx right 1 
                    0x0006 => {
                        opstr = "SHR";
                        // Set VF if lsb = 1
                        self.v[0xF] = self.v[x] & 1;
                        self.v[x] >>= 1;
                        println!("{} Vx >>= 1", opstr);
                    }
                    // 8xy7: SUBN Vx = Vy - Vx
                    0x0007 => {
                        opstr = "SUBN";
                        let Wrapping(_vyx) = Wrapping(self.v[y]) - Wrapping(self.v[x]);
                        self.v[x] = _vyx;
                        println!("{} Vx = Vy({:#06X}) - Vx({:#06X})", opstr, self.v[y], self.v[x]);
                    }
                    // 8xyE: SHL - Shift Vx left 1 
                    0x000E => {
                        // Set VF if lsb = 1
                        self.v[0xF] = self.v[x] >> MSB_POS;
                        self.v[x] <<= 1;
                        println!("{} Vx <<= 1", opstr);
                    }
                    _ => {
                        eprintln!("Invalid 0x8000 opcode: {:#06X}", self.opcode);
                    }
                }
            }
            // 9xy0: SNE - Skip next instruction if Vx != Vy
            0x9000 => {
                opstr = "SNE";
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                    println!("{} Vx != Vy", opstr);
                    println!("SKIPPING INSTRUCTION");
                } else {
                    println!("{} Vx == Vy", opstr);
                    println!("NOT SKIPPING INSTRUCTION");
                }
            }
            // Annn: LD - Set I to nnn
            0xA000 => {
                opstr = "LD";
                self.i = nnn;
                println!("{} I = nnn({:#3X})", opstr, nnn);
            }
            // Bnnn: JP - Jump to nnn + V0
            0xB000 => {
                opstr = "JP";
                self.pc = self.v[0x0] as usize + nnn as usize;
                self.pc -= 2;
                println!("{} nnn({:#05X}) + V0({:#06X})", opstr, nnn, self.v[0x0]);
            }
            // Cxkk - RND - Generate random number from 0-255, then & kk and store the result in Vx
            0xC000 => {
                opstr = "RND";
                self.v[x] = rand::thread_rng().gen_range(0..0xFF) & kk;
                println!("{} kk({:#04X}) = {:#04X}", opstr, kk, self.v[x]);
            }
            // Dxyn: DRW - Draw
            0xD000 => {
                opstr = "DRW";
                self.v[0xF] = 0x0;
                // Reduce if overflow
                if self.v[x] > DISP_X as u8 {
                    println!("Performing modulo reduction for x-axis");
                    self.v[x] %= DISP_X as u8; 
                }
                if self.v[y] > DISP_Y as u8 {
                    println!("Performing modulo reduction for y-axis");
                    self.v[y] %= DISP_Y as u8; 
                }

                for dy in 0..n as usize {
                    let px = self.mem[self.i as usize + dy];
                    for dx in 0..8 as usize {
                        if px & (0x80 >> dx) != 0 {
                            // If a pixel is drawn to an already drawn pixel, it is unset
                            if self.gfx[(self.v[x] as usize + dx + ((self.v[y] as usize + dy) * DISP_X))] != 0{
                                // Indicate that pixel was unset
                                self.v[0xF] = 1;
                            }
                            self.gfx[self.v[x] as usize + dx + ((self.v[y] as usize + dy) * DISP_X)] ^= 1;
                        }
                    }
                }
                println!("{} Vx({:#06X}) Vy({:#06X})", opstr, self.v[y], self.v[x]);
                self.draw_flag = true;
            }
            0xE000 => {
                match self.opcode & 0x000F {
                    // Ex9E: SKP - Skip next instruction if key with Vx is pressed
                    0x000E => {
                        opstr = "SKP";
                        if self.keys[self.v[x] as usize] {
                            println!("{} Key[Vx({:#X})] is pressed", opstr, self.v[x]);
                            println!("SKIPPING INSTRUCTION");
                            self.pc += 2;
                        } else {
                            println!("{} Key[Vx({:#X})] is not pressed", opstr, self.v[x]);
                            println!("NOT SKIPPING INSTRUCTION");
                        }
                    }
                    // ExA1: SKNP - Skip next instruction if key with Vx is not presed
                    0x0001 => {
                        if !self.keys[self.v[x] as usize] {
                            println!("{} Key[Vx({:#X})] is not pressed", opstr, self.v[x]);
                            println!("SKIPPING INSTRUCTION");
                            self.pc += 2;
                        } else {
                            println!("{} Key[Vx({:#X})] is pressed", opstr, self.v[x]);
                            println!("NOT SKIPPING INSTRUCTION");
                        }
                    }
                    _ => {
                        eprintln!("Invalid 0xE000 opcode {:#06X}", self.opcode);
                    }
                }
            }
            0xF000 => {
                match self.opcode & 0x00FF {
                    
                    // Fx07: LD Vx, DT - Set Vx = delay timer value.
                    0x0007 => {
                        opstr = "LD";
                        self.v[x] = self.dt;
                        println!("{} Vx = dt({:#04X})", opstr, self.dt);
                    }
                    // TODO
                    // Fx0A: LD Vx, K - Wait for a key press and store the value of the key in Vx.
                    0x000A => {
                        opstr = "LD";
                        let key_idx = input_handler.wait_for_key(self, sdl_context);
                        self.v[x] = key_idx;
                        println!("{} Vx, K{:#06X}", opstr, key_idx);
                    }
                    // Fx15: LD DT, Vx - Set delay timer = Vx.
                    0x0015 => {
                        opstr = "LD";
                        self.dt = self.v[x];
                        println!("{} dt = Vx({:#06X})", opstr, self.dt);
                    }
                    // Fx18: LD ST, Vx - Set sound timer = Vx.
                    0x0018 => {
                        opstr = "LD";
                        self.st = self.v[x];
                        println!("{} st = Vx({:#06X})", opstr, self.v[x]);
                    }
                    // Fx1E: ADD I, Vx ---- I += Vx
                    0x001E => {
                        opstr = "ADD";
                        self.i += self.v[x] as u16;
                        println!("{} i += Vx({:#06X})", opstr, self.v[x]);
                    }
                    // Fx29: LD F, Vx - Set I = location of sprite for digit Vx.
                    0x0029 => {
                        opstr = "LD";
                        self.i = self.v[x] as u16 * 0x5;
                        println!("{} i = Vx({:#05X}) * 0x5", opstr, self.v[x]);
                    }
                    // Fx33: LD B, Vx - Store BCD representation of Vx in memory locations I, I+1, and I+2.
                    0x0033 => {
                        opstr = "LD";
                        // Load 100s place
                        self.mem[self.i as usize] = self.v[x] / 100;
                        // Load 10s place
                        self.mem[self.i as usize + 1] = (self.v[x] / 10) % 10;
                        // Loads 1s place
                        self.mem[self.i as usize + 2] = self.v[x] % 10;
                        println!("{} mem = Vx BCD", opstr);
                    }
                    // Fx55: LD [I], Vx - Store registers V0 through Vx in memory starting at location I.
                    0x0055 => {
                        opstr = "LD";
                        for i in 0..x+1 {
                            self.mem[self.i as usize + i] = self.v[i];
                        }
                        println!("{} mem = V0-Vx({:#04X}) + 0x5", opstr, x);
                    }
                    // Fx65: LD Vx, [I] - Read from memory starting at location I and store it into registers V0 through Vx.
                    0x0065 => {
                        opstr = "LD";
                        for i in 0..x+1 {
                           self.v[i] = self.mem[self.i as usize + i]; 
                        }
                        println!("{} V0-Vx({:#X}) = mem", opstr, x);
                    }
                    _ => {
                        eprintln!("Invalid 0xF000 opcode {:#X}", self.opcode);  
                    }
                }
            }
            _ => eprintln!("Invalid opcode {:#X}", self.opcode),
        }
    }
    
    pub fn print_registers(self: &Self){
        println!("------------------------");
        println!("V REGISTERS:");
        for i in 0..NUM_VREGS {
            print!("{:#5X}", i);
        }
        println!();

        for i in 0..NUM_VREGS {
            print!("{:#5X}", self.v[i]);
        }
        println!();

        println!("I: {:#X}", self.i);
        println!("DT: {:#X}", self.dt);
        println!("ST: {:#X}", self.st);
    }

    pub fn print_screen(self: &mut Self){
        if self.draw_flag {
            for i in 0..DISP_X*DISP_Y {
                if self.gfx[i] != 0 {
                    print!("{}", PX);
                } else {
                    print!("  ");
                }
                if (i + 1) % DISP_X == 0 {
                    println!();
                }
            }
        }
    }
}
