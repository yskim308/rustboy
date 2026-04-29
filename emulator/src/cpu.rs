use crate::{bus::Bus, cpu::register::Registers};

#[cfg(test)]
mod cpu_tests;

mod op_jumps;
mod op_memload;
mod op_rload;

pub mod register;

pub struct Cpu {
    registers: Registers,
    ime: bool,
    // todo: Other CPU state like interrupts
}

impl Cpu {
    pub fn new() -> Self {
        let registers = Registers::new();
        Self {
            registers,
            ime: false,
        }
    }

    fn advance_pc(&mut self) {
        self.registers.pc = self.registers.pc.wrapping_add(1);
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

    // ============= stack helpers ================
    fn push_u8(&mut self, bus: &mut Bus, data: u8) {
        self.registers.decrement_sp();
        bus.write_u8(self.registers.sp, data);
    }

    fn push_u16(&mut self, bus: &mut Bus, data: u16) {
        let [low, high] = data.to_le_bytes();
        self.push_u8(bus, high);
        self.push_u8(bus, low);
    }

    fn pop_u8(&mut self, bus: &mut Bus) -> u8 {
        let value = bus.read_u8(self.registers.sp);
        self.registers.increment_sp();

        value
    }

    fn pop_u16(&mut self, bus: &mut Bus) -> u16 {
        let low = self.pop_u8(bus);
        let high = self.pop_u8(bus);
        u16::from_le_bytes([low, high])
    }

    pub fn execute(&mut self, opcode: u8, bus: &mut Bus) -> u8 {
        // https://izik1.github.io/gbops/
        // every opcode is developed through TDD, see cpu/tests
        match opcode {
            0x00 => 4,
            0x01 => self.ld_bc_u16(bus),
            0x02 => self.ld_at_bc_a(bus),
            0x06 => self.ld_b_u8(bus),
            0x0A => self.ld_a_at_bc(bus),
            0x0E => self.ld_c_u8(bus),
            0x11 => self.ld_de_u16(bus),
            0x12 => self.ld_at_de_a(bus),
            0x16 => self.ld_d_u8(bus),
            0x18 => self.jr_i8(bus),
            0x1A => self.ld_a_at_de(bus),
            0x1E => self.ld_e_u8(bus),
            0x20 => self.jr_nz_i8(bus),
            0x21 => self.ld_hl_u16(bus),
            0x22 => self.ld_at_hli_a(bus),
            0x28 => self.jr_z_i8(bus),
            0x2A => self.ld_a_hli(bus),
            0x26 => self.ld_h_u8(bus),
            0x2E => self.ld_l_u8(bus),
            0x30 => self.jr_nc_i8(bus),
            0x31 => self.ld_sp_u16(bus),
            0x32 => self.ld_at_hld_a(bus),
            0x36 => self.ld_at_hl_u8(bus),
            0x38 => self.jr_c_i8(bus),
            0x3A => self.ld_a_at_hld(bus),
            0x3E => self.ld_a_u8(bus),
            0x40 => self.ld_b_b(),
            0x41 => self.ld_b_c(),
            0x42 => self.ld_b_d(),
            0x43 => self.ld_b_e(),
            0x44 => self.ld_b_h(),
            0x45 => self.ld_b_l(),
            0x46 => self.ld_b_at_hl(bus),
            0x47 => self.ld_b_a(),
            0x48 => self.ld_c_b(),
            0x49 => self.ld_c_c(),
            0x4A => self.ld_c_d(),
            0x4B => self.ld_c_e(),
            0x4C => self.ld_c_h(),
            0x4D => self.ld_c_l(),
            0x4E => self.ld_c_at_hl(bus),
            0x4F => self.ld_c_a(),
            0x50 => self.ld_d_b(),
            0x51 => self.ld_d_c(),
            0x52 => self.ld_d_d(),
            0x53 => self.ld_d_e(),
            0x54 => self.ld_d_h(),
            0x55 => self.ld_d_l(),
            0x56 => self.ld_d_at_hl(bus),
            0x57 => self.ld_d_a(),
            0x58 => self.ld_e_b(),
            0x59 => self.ld_e_c(),
            0x5A => self.ld_e_d(),
            0x5B => self.ld_e_e(),
            0x5C => self.ld_e_h(),
            0x5D => self.ld_e_l(),
            0x5E => self.ld_e_at_hl(bus),
            0x5F => self.ld_e_a(),
            0x60 => self.ld_h_b(),
            0x61 => self.ld_h_c(),
            0x62 => self.ld_h_d(),
            0x63 => self.ld_h_e(),
            0x64 => self.ld_h_h(),
            0x65 => self.ld_h_l(),
            0x66 => self.ld_h_at_hl(bus),
            0x67 => self.ld_h_a(),
            0x68 => self.ld_l_b(),
            0x69 => self.ld_l_c(),
            0x6A => self.ld_l_d(),
            0x6B => self.ld_l_e(),
            0x6C => self.ld_l_h(),
            0x6D => self.ld_l_l(),
            0x6E => self.ld_l_at_hl(bus),
            0x6F => self.ld_l_a(),
            0x70 => self.ld_at_hl_b(bus),
            0x71 => self.ld_at_hl_c(bus),
            0x72 => self.ld_at_hl_d(bus),
            0x73 => self.ld_at_hl_e(bus),
            0x74 => self.ld_at_hl_h(bus),
            0x75 => self.ld_at_hl_l(bus),
            0x77 => self.ld_at_hl_a(bus),
            0x78 => self.ld_a_b(),
            0x79 => self.ld_a_c(),
            0x7A => self.ld_a_d(),
            0x7B => self.ld_a_e(),
            0x7C => self.ld_a_h(),
            0x7D => self.ld_a_l(),
            0x7E => self.ld_a_at_hl(bus),
            0x7F => self.ld_a_a(),
            0xC0 => self.ret_nz(bus),
            0xC2 => self.jp_nz_u16(bus),
            0xC3 => self.jp_u16(bus),
            0xC4 => self.call_nz_u16(bus),
            0xC7 => self.rst(bus, 0x00),
            0xC8 => self.ret_z(bus),
            0xC9 => self.ret(bus),
            0xCA => self.jp_z_u16(bus),
            0xCC => self.call_z_u16(bus),
            0xCD => self.call_u16(bus),
            0xCF => self.rst(bus, 0x08),
            0xD0 => self.ret_nc(bus),
            0xD2 => self.jp_nc_u16(bus),
            0xD4 => self.call_nc_u16(bus),
            0xD7 => self.rst(bus, 0x10),
            0xD8 => self.ret_c(bus),
            0xD9 => self.reti(bus),
            0xDA => self.jp_c_u16(bus),
            0xDC => self.call_c_u16(bus),
            0xDF => self.rst(bus, 0x18),
            0xE7 => self.rst(bus, 0x20),
            0xE9 => self.jp_hl(bus),
            0xEF => self.rst(bus, 0x28),
            0xF7 => self.rst(bus, 0x30),
            0xF9 => self.ld_sp_hl(),
            0xFF => self.rst(bus, 0x38),
            _ => panic!(
                "Unimplemented opcode: {:#04X} at pc {:#06X}",
                opcode, self.registers.pc
            ),
        }
    }
}
