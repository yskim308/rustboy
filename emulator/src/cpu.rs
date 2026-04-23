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
        self.registers.pc += self.registers.pc.wrapping_add(1);
    }

    pub fn step(&mut self, bus: &mut Bus) -> u8 {
        let read_byte = bus.read_u8(self.registers.pc);
        self.advance_pc();
        self.execute(read_byte, bus)
    }

    fn fetch_u8(&mut self, bus: &mut Bus) -> u8 {
        let value = bus.read_u8(self.registers.pc);
        self.advance_pc();
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
            0x01 => self.ld_bc_u16(bus),
            0x06 => self.ld_b_u8(bus),
            0x0E => self.ld_c_u8(bus),
            0x11 => self.ld_de_u16(bus),
            0x16 => self.ld_d_u8(bus),
            0x1E => self.ld_e_u8(bus),
            0x21 => self.ld_hl_u16(bus),
            0x2A => self.ld_a_hli(bus),
            0x26 => self.ld_h_u8(bus),
            0x2E => self.ld_l_u8(bus),
            0x31 => self.ld_sp_u16(bus),
            0x3E => self.ld_a_u8(bus),
            0x40 => self.ld_b_b(),
            0x41 => self.ld_b_c(),
            0x42 => self.ld_b_d(),
            0x43 => self.ld_b_e(),
            0x44 => self.ld_b_h(),
            0x45 => self.ld_b_l(),
            0x47 => self.ld_b_a(),
            0x48 => self.ld_c_b(),
            0x49 => self.ld_c_c(),
            0x4A => self.ld_c_d(),
            0x4B => self.ld_c_e(),
            0x4C => self.ld_c_h(),
            0x4D => self.ld_c_l(),
            0x4F => self.ld_c_a(),
            0x50 => self.ld_d_b(),
            0x51 => self.ld_d_c(),
            0x52 => self.ld_d_d(),
            0x53 => self.ld_d_e(),
            0x54 => self.ld_d_h(),
            0x55 => self.ld_d_l(),
            0x57 => self.ld_d_a(),
            0x58 => self.ld_e_b(),
            0x59 => self.ld_e_c(),
            0x5A => self.ld_e_d(),
            0x5B => self.ld_e_e(),
            0x5C => self.ld_e_h(),
            0x5D => self.ld_e_l(),
            0x5F => self.ld_e_a(),
            0x60 => self.ld_h_b(),
            0x61 => self.ld_h_c(),
            0x62 => self.ld_h_d(),
            0x63 => self.ld_h_e(),
            0x64 => self.ld_h_h(),
            0x65 => self.ld_h_l(),
            0x67 => self.ld_h_a(),
            0x68 => self.ld_l_b(),
            0x69 => self.ld_l_c(),
            0x6A => self.ld_l_d(),
            0x6B => self.ld_l_e(),
            0x6C => self.ld_l_h(),
            0x6D => self.ld_l_l(),
            0x6F => self.ld_l_a(),
            0x78 => self.ld_a_b(),
            0x79 => self.ld_a_c(),
            0x7A => self.ld_a_d(),
            0x7B => self.ld_a_e(),
            0x7C => self.ld_a_h(),
            0x7D => self.ld_a_l(),
            0x7F => self.ld_a_a(),
            0xC3 => {
                let value = self.fetch_u16(bus);
                self.registers.pc = value;
                16
            }
            0xF9 => self.ld_sp_hl(),
            _ => panic!(
                "Unimplemented opcode: {:#04X} at pc {:#06X}",
                opcode, self.registers.pc
            ),
        }
    }
}
