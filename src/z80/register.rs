use std::path::MAIN_SEPARATOR;

pub struct Registers {
    main_regs: [u16; 4],
    alt_regs: [u16; 4],
    index_x: u16,
    index_y: u16,
    stack_pointer: u16,
    interrupt: u8,
    refresh: u8,
    prog_counter: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers { 
            main_regs: [0; 4],
            alt_regs: [0; 4],
            index_x: 0,
            index_y: 0,
            stack_pointer: 0,
            interrupt: 0,
            refresh: 0,
            prog_counter: 0,
        }
    }

    pub fn get(&self, name: &str) -> Option<u16> {
        match name {
            "AF" => Some(self.main_regs[0]),
            "A" => Some((self.main_regs[0] & 0xff00) >> 8),
            "BC" => Some(self.main_regs[1]),
            "B" => Some((self.main_regs[1] & 0xff00) >> 8),
            "C" => Some(self.main_regs[1] & 0x00ff),
            "DE" => Some(self.main_regs[2]),
            "D" => Some((self.main_regs[2] & 0xff00) >> 8),
            "E" => Some(self.main_regs[2] & 0x00ff),
            "HL" => Some(self.main_regs[3]),
            "H" => Some((self.main_regs[3] & 0xff00) >> 8),
            "L" => Some(self.main_regs[3] & 0x00ff),
            _ => None,
        }
    }
}

