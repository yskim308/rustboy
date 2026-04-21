use crate::{bus::Bus, cpu::register::Registers};

pub mod register;

pub struct Cpu {
    registers: Registers,
    // todo: Other CPU state like interrupts
}

impl Cpu {
    pub fn new() -> Self {
        let registers = Registers::new();
        Self { registers }
    }

    fn advance_pc(&mut self) {
        self.registers.pc += 1;
    }

    pub fn step(&mut self, bus: &mut Bus) -> u8 {
        let read_byte = bus.read_u8(self.registers.pc);
        self.advance_pc();
        self.execute(read_byte, bus)
    }

    pub fn execute(&mut self, opcode: u8, bus: &mut Bus) -> u8 {
        match opcode {
            0x00 => 4,
            _ => panic!("Unimplemented opcode: {opcode} at pc {}", self.registers.pc),
        }
    }
}
