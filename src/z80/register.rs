#[derive(Debug)]
pub enum Register {
    A,
    A1,
    F,
    F1,
    BC,
    BC1,
    B,
    B1,
    C,
    C1,
    DE,
    DE1,
    D,
    D1,
    E,
    E1,
    HL,
    HL1,
    H,
    H1,
    L,
    L1,
    IX,
    IY,
    SP,
    I,
    R,
    PC,
}

#[inline]
fn high_reg(reg: u16) -> u16 {
    (reg & 0xff00) >> 8
}

#[inline]
fn set_high_reg(reg: u16, value: u16) -> u16 {
    (reg & 0x00ff) | ((value & 0x00ff) << 8)
}

#[inline]
fn low_reg(reg: u16) -> u16 {
    reg & 0x00ff
}

#[inline]
fn set_low_reg(reg: u16, value: u16) -> u16 {
    (reg & 0xff00) | (value & 0x00ff)
}

#[derive(Default)]
pub struct RegisterSet {
    af: u16,
    af1: u16,
    bc: u16,
    bc1: u16,
    de: u16,
    de1: u16,
    hl: u16,
    hl1: u16,
    ix: u16,
    iy: u16,
    sp: u16,
    i: u8,
    r: u8,
    pc: u16,
}

impl RegisterSet {
    pub fn new() -> RegisterSet {
        RegisterSet {
            ..Default::default()
        }
    }

    pub fn get_reg(&self, reg: Register) -> u16 {
        match reg {
            Register::A => high_reg(self.af),
            Register::A1 => high_reg(self.af1),
            Register::F => low_reg(self.af),
            Register::F1 => low_reg(self.af1),
            Register::BC => self.bc,
            Register::BC1 => self.bc1,
            Register::B => high_reg(self.bc),
            Register::B1 => high_reg(self.bc1),
            Register::C => low_reg(self.bc),
            Register::C1 => low_reg(self.bc1),
            Register::DE => self.de,
            Register::DE1 => self.de1,
            Register::D => high_reg(self.de),
            Register::D1 => high_reg(self.de1),
            Register::E => low_reg(self.de),
            Register::E1 => low_reg(self.de1),
            Register::HL => self.hl,
            Register::HL1 => self.hl1,
            Register::H => high_reg(self.hl),
            Register::H1 => high_reg(self.hl1),
            Register::L => low_reg(self.hl),
            Register::L1 => low_reg(self.hl1),
            Register::IX => self.ix,
            Register::IY => self.iy,
            Register::SP => self.sp,
            Register::I => self.i as u16,
            Register::R => self.r as u16,
            Register::PC => self.pc,
        }
    }

    pub fn set_reg(&mut self, reg: Register, value: u16) {
        match reg {
            Register::A => self.af = set_high_reg(self.af, value),
            Register::A1 => self.af1 = set_high_reg(self.af1, value),
            Register::F => self.af = set_low_reg(self.af, value),
            Register::F1 => self.af1 = set_low_reg(self.af1, value),
            Register::BC => self.bc = value,
            Register::BC1 => self.bc1 = value,
            Register::B => self.af = set_high_reg(self.bc, value),
            Register::B1 => self.af1 = set_high_reg(self.bc1, value),
            Register::C => self.af = set_low_reg(self.bc, value),
            Register::C1 => self.af1 = set_low_reg(self.bc1, value),
            Register::DE => self.de = value,
            Register::DE1 => self.de1 = value,
            Register::D => self.af = set_high_reg(self.de, value),
            Register::D1 => self.af1 = set_high_reg(self.de1, value),
            Register::E => self.af = set_low_reg(self.de, value),
            Register::E1 => self.af1 = set_low_reg(self.de1, value),
            Register::HL => self.hl = value,
            Register::HL1 => self.de1 = value,
            Register::H => self.af = set_high_reg(self.hl, value),
            Register::H1 => self.af1 = set_high_reg(self.hl1, value),
            Register::L => self.af = set_low_reg(self.hl, value),
            Register::L1 => self.af1 = set_low_reg(self.hl1, value),
            Register::IX => self.ix = value,
            Register::IY => self.iy = value,
            Register::SP => self.sp = value,
            Register::I => self.i = (value & 0x00ff) as u8,
            Register::R => self.r = (value & 0x00ff) as u8,
            Register::PC => self.pc = value,
        }
    }
}
