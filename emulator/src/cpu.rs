use crate::{bus::Bus, cpu::register::Registers};

#[cfg(test)]
mod cpu_tests;

mod op_rload;
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

    fn fetch_u8(&mut self, bus: &mut Bus) -> u8 {
        let value = bus.read_u8(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        value
    }

    fn fetch_u16(&mut self, bus: &mut Bus) -> u16 {
        let low = self.fetch_u8(bus);
        let high = self.fetch_u8(bus);
        let value = u16::from_le_bytes([low, high]);
        value
    }

    pub fn execute(&mut self, opcode: u8, bus: &mut Bus) -> u8 {
        // https://izik1.github.io/gbops/
        // every opcode is developed through TDD, see cpu/tests
        match opcode {
            0x00 => 4,
            0x0E => self.ld_c_u8(bus),
            0x11 => self.ld_de_u16(bus),
            0x21 => self.ld_hl_u16(bus),
            0x2a => self.ld_a_hli(bus),
            0x40 => self.ld_b_b(),
            0x41 => self.ld_b_c(),
            0x47 => self.ld_b_a(),
            0xC3 => {
                let value = self.fetch_u16(bus);
                self.registers.pc = value;
                16
            }
            _ => panic!(
                "Unimplemented opcode: {:#04X} at pc {:#06X}",
                opcode, self.registers.pc
            ),
        }
    }
}
