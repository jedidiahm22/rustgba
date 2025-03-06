use crate::memory::Ram;
use crate::memory::MemmoryMapper;

pub const FLAG_Z_MASK: u8 = 0b10000000;
pub const FLAG_N: u8 = 1 << 0b01000000;
pub const FLAG_H: u8 = 1 << 0b00100000;
pub const FLAG_C: u8 = 1 << 0b00010000;

pub enum Flags {
    Z,
    N,
    H,
    C,
}

pub enum Registers {
    A = 0,
    B,
    C,
    D,
    E,
    H,
    L,
    SP,
    PC,
    HL,
    AF,
    BC.
    DE,
}

pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    flags: u8,
    memmap: MemmoryMapper,
    max_stack_size: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0xEDFF,
            pc: 0,
            flags: 0,
            memmap: MemmoryMapper::new(),
            max_stack_size: 0xEDFF - 0xE000,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.d = 0;
        self.e = 0;
        self.h = 0;
        self.l = 0;
        self.sp = 0xEDFF;
        self.pc = 0;
        self.flags = 0;
    }

    pub fn get_register(&self, reg: Registers) -> u8 {
        match reg {
            Registers::A => self.a,
            Registers::B => self.b,
            Registers::C => self.c,
            Registers::D => self.d,
            Registers::E => self.e,
            Registers::H => self.h,
            Registers::L => self.l,
            Registers::SP => self.sp as u8,
            Registers::PC => self.pc as u8,
        }
    }

    pub fn get_register_from_byte(&self, byte: u8) -> Registers {
        match byte {
            0x00 => Registers::B,
            0x01 => Registers::C,
            0x02 => Registers::D,
            0x03 => Registers::E,
            0x04 => Registers::H,
            0x05 => Registers::L,
            0x06 => Registers::HL,
            0x07 => Registers::A,
            _ => Registers::A,
        }
    }

    pub fn set_register(&mut self, reg: Registers, value: u8) {
        match reg {
            Registers::A => self.a = value,
            Registers::B => self.b = value,
            Registers::C => self.c = value,
            Registers::D => self.d = value,
            Registers::E => self.e = value,
            Registers::H => self.h = value,
            Registers::L => self.l = value,
            Registers::SP => self.sp = value as u16,
            Registers::PC => self.pc = value as u16,
        }
    }

    pub fn get_register(&self, reg: Registers) -> u8 {
        match reg {
            Registers::A => self.a,
            Registers::B => self.b,
            Registers::C => self.c,
            Registers::D => self.d,
            Registers::E => self.e,
            Registers::H => self.h,
            Registers::L => self.l,
            Registers::SP => self.sp as u8,
            Registers::PC => self.pc as u8,
        }
    }

    pub fn get_register_u16(&self, reg: Registers) -> u16 {
        match reg {
            Registers::AF => self.a as u16,
            Registers::BC => (self.b as u16)<<8 | (self.c as u16),
            Registers::DE => (self.d as u16)<<8 | (self.e as u16),
            Registers::HL => (self.h as u16)<<8 | (self.l as u16),
            Registers::SP => self.sp,
            Registers::PC => self.pc,
        }
    }

    pub fn fetch(&mut self) -> u8 {
        let value = self.memmap.read(self.pc);
        self.pc += 1;
        return value;
    }

    pub fn push(&mut self, value: u8) {
        self.sp -= 1;
        self.memmap.write(self.sp, value);
    }

    pub fn pop(&mut self) -> u8 {
        let value = self.memmap.read(self.sp);
        self.sp += 1;
        return value;
    }

    pub fn step(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
    }

    pub fn execute(&mut self, u8 opcode) {}
}