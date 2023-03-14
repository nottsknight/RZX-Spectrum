use self::register::RegisterSet;

mod opcode;
mod register;

const ROM_SIZE: usize = 16 * 1024;

const RAM_SIZE: usize = 48 * 1024;

pub struct Z80Cpu {
    rom: [u8; ROM_SIZE],
    ram: [u8; RAM_SIZE],
    registers: RegisterSet,
}

impl Z80Cpu {
    pub fn new() -> Z80Cpu {
        Z80Cpu { 
            rom: [0; ROM_SIZE],
            ram: [0; RAM_SIZE],
            registers: RegisterSet::new(),
        }
    }
}
