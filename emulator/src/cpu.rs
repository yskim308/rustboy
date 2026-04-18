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

    pub fn execute(&mut self, opcode: u8, bus: &mut Bus) {
        match opcode {
            _ => panic!("Unimplemented opcode: {opcode} at pc {}", self.registers.pc),
        }
    }
}
