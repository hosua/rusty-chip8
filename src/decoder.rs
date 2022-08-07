pub struct Decoder;
impl Decoder {
    pub fn x(_opcode: u16) -> usize {
        return ((_opcode & 0xF00) >> 8) as usize;
    }
    pub fn y(_opcode: u16) -> usize {
        return ((_opcode & 0x00F0) >> 4) as usize;
    }
    pub fn kk(_opcode: u16) -> u8 {
        return (_opcode & 0x00FF) as u8;
    }
    pub fn nnn(_opcode: u16) -> u16 {
        return (_opcode & 0xFFF) as u16;
    }
    pub fn n(_opcode: u16) -> u8 {
        return (_opcode & 0x000F) as u8;
    }
}
