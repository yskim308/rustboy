use crate::{bus::Bus, cpu::register::Registers};

#[cfg(test)]
mod cpu_tests;

mod op_alu16;
mod op_alu8;
mod op_jumps;
mod op_memload;
mod op_rload;
mod op_rsb;
mod op_special;
mod op_stack;

pub mod register;

pub struct Cpu {
    registers: Registers,
    ime: bool,
    halt: bool,
    halt_bug: bool,
    ime_pending: bool,
    // todo: Other CPU state like interrupts
}

impl Cpu {
    pub fn new() -> Self {
        let registers = Registers::new();
        Self {
            registers,
            ime: false,
            halt: false,
            halt_bug: false,
            ime_pending: false,
        }
    }

    fn advance_pc(&mut self) {
        self.registers.pc = self.registers.pc.wrapping_add(1);
    }

    pub fn step(&mut self, bus: &mut Bus) -> u8 {
        let read_byte = bus.read_u8(self.registers.pc);
        if self.halt_bug {
            self.halt_bug = false;
        } else {
            self.advance_pc();
        }

        let cycles = self.execute(read_byte, bus);

        if self.ime_pending {
            self.ime_pending = false;
            self.ime = true;
        }

        cycles
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
            0x00 => self.nop(),
            0x01 => self.ld_bc_u16(bus),
            0x02 => self.ld_at_bc_a(bus),
            0x03 => self.inc_bc(),
            0x04 => self.inc_b(),
            0x05 => self.dec_b(),
            0x06 => self.ld_b_u8(bus),
            0x07 => self.rlca(),
            0x09 => self.add_hl_bc(),
            0x0A => self.ld_a_at_bc(bus),
            0x0B => self.dec_bc(),
            0x0C => self.inc_c(),
            0x0D => self.dec_c(),
            0x0E => self.ld_c_u8(bus),
            0x0F => self.rrca(),
            0x10 => self.stop(bus),
            0x11 => self.ld_de_u16(bus),
            0x12 => self.ld_at_de_a(bus),
            0x13 => self.inc_de(),
            0x14 => self.inc_d(),
            0x15 => self.dec_d(),
            0x16 => self.ld_d_u8(bus),
            0x17 => self.rla(),
            0x18 => self.jr_i8(bus),
            0x19 => self.add_hl_de(),
            0x1A => self.ld_a_at_de(bus),
            0x1B => self.dec_de(),
            0x1C => self.inc_e(),
            0x1D => self.dec_e(),
            0x1E => self.ld_e_u8(bus),
            0x1F => self.rra(),
            0x20 => self.jr_nz_i8(bus),
            0x21 => self.ld_hl_u16(bus),
            0x22 => self.ld_at_hli_a(bus),
            0x23 => self.inc_hl(),
            0x24 => self.inc_h(),
            0x25 => self.dec_h(),
            0x26 => self.ld_h_u8(bus),
            0x27 => self.daa(),
            0x28 => self.jr_z_i8(bus),
            0x29 => self.add_hl_hl(),
            0x2A => self.ld_a_hli(bus),
            0x2B => self.dec_hl(),
            0x2C => self.inc_l(),
            0x2D => self.dec_l(),
            0x2E => self.ld_l_u8(bus),
            0x2F => self.cpl(),
            0x30 => self.jr_nc_i8(bus),
            0x31 => self.ld_sp_u16(bus),
            0x32 => self.ld_at_hld_a(bus),
            0x33 => {
                self.registers.increment_sp();
                8
            }
            0x34 => self.inc_at_hl(bus),
            0x35 => self.dec_at_hl(bus),
            0x36 => self.ld_at_hl_u8(bus),
            0x37 => self.scf(),
            0x38 => self.jr_c_i8(bus),
            0x39 => self.add_hl_sp(),
            0x3A => self.ld_a_at_hld(bus),
            0x3B => {
                self.registers.decrement_sp();
                8
            }
            0x3C => self.inc_a(),
            0x3D => self.dec_a(),
            0x3E => self.ld_a_u8(bus),
            0x3F => self.ccf(),
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
            0x76 => self.halt(bus),
            0x77 => self.ld_at_hl_a(bus),
            0x78 => self.ld_a_b(),
            0x79 => self.ld_a_c(),
            0x7A => self.ld_a_d(),
            0x7B => self.ld_a_e(),
            0x7C => self.ld_a_h(),
            0x7D => self.ld_a_l(),
            0x7E => self.ld_a_at_hl(bus),
            0x7F => self.ld_a_a(),
            0x80 => self.add_a_b(),
            0x81 => self.add_a_c(),
            0x82 => self.add_a_d(),
            0x83 => self.add_a_e(),
            0x84 => self.add_a_h(),
            0x85 => self.add_a_l(),
            0x86 => self.add_a_at_hl(bus),
            0x87 => self.add_a_a(),
            0x88 => self.adc_a_b(),
            0x89 => self.adc_a_c(),
            0x8A => self.adc_a_d(),
            0x8B => self.adc_a_e(),
            0x8C => self.adc_a_h(),
            0x8D => self.adc_a_l(),
            0x8E => self.adc_a_at_hl(bus),
            0x8F => self.adc_a_a(),
            0x90 => self.sub_a_b(),
            0x91 => self.sub_a_c(),
            0x92 => self.sub_a_d(),
            0x93 => self.sub_a_e(),
            0x94 => self.sub_a_h(),
            0x95 => self.sub_a_l(),
            0x96 => self.sub_a_at_hl(bus),
            0x97 => self.sub_a_a(),
            0x98 => self.sbc_a_b(),
            0x99 => self.sbc_a_c(),
            0x9A => self.sbc_a_d(),
            0x9B => self.sbc_a_e(),
            0x9C => self.sbc_a_h(),
            0x9D => self.sbc_a_l(),
            0x9E => self.sbc_a_at_hl(bus),
            0x9F => self.sbc_a_a(),
            0xA0 => self.and_a_b(),
            0xA1 => self.and_a_c(),
            0xA2 => self.and_a_d(),
            0xA3 => self.and_a_e(),
            0xA4 => self.and_a_h(),
            0xA5 => self.and_a_l(),
            0xA6 => self.and_a_at_hl(bus),
            0xA7 => self.and_a_a(),
            0xA8 => self.xor_a_b(),
            0xA9 => self.xor_a_c(),
            0xAA => self.xor_a_d(),
            0xAB => self.xor_a_e(),
            0xAC => self.xor_a_h(),
            0xAD => self.xor_a_l(),
            0xAE => self.xor_a_at_hl(bus),
            0xAF => self.xor_a_a(),
            0xB0 => self.or_a_b(),
            0xB1 => self.or_a_c(),
            0xB2 => self.or_a_d(),
            0xB3 => self.or_a_e(),
            0xB4 => self.or_a_h(),
            0xB5 => self.or_a_l(),
            0xB6 => self.or_a_at_hl(bus),
            0xB7 => self.or_a_a(),
            0xB8 => self.cp_a_b(),
            0xB9 => self.cp_a_c(),
            0xBA => self.cp_a_d(),
            0xBB => self.cp_a_e(),
            0xBC => self.cp_a_h(),
            0xBD => self.cp_a_l(),
            0xBE => self.cp_a_at_hl(bus),
            0xBF => self.cp_a_a(),
            0xC0 => self.ret_nz(bus),
            0xC1 => self.pop_bc(bus),
            0xC2 => self.jp_nz_u16(bus),
            0xC3 => self.jp_u16(bus),
            0xC4 => self.call_nz_u16(bus),
            0xC5 => self.push_bc(bus),
            0xC6 => self.add_a_fetch_u8(bus),
            0xC7 => self.rst(bus, 0x00),
            0xC8 => self.ret_z(bus),
            0xC9 => self.ret(bus),
            0xCA => self.jp_z_u16(bus),
            0xCB => self.execute_cb(bus),
            0xCC => self.call_z_u16(bus),
            0xCD => self.call_u16(bus),
            0xCE => self.adc_a_fetch_u8(bus),
            0xCF => self.rst(bus, 0x08),
            0xD0 => self.ret_nc(bus),
            0xD1 => self.pop_de(bus),
            0xD2 => self.jp_nc_u16(bus),
            0xD4 => self.call_nc_u16(bus),
            0xD5 => self.push_de(bus),
            0xD6 => self.sub_a_fetch_u8(bus),
            0xD7 => self.rst(bus, 0x10),
            0xD8 => self.ret_c(bus),
            0xD9 => self.reti(bus),
            0xDA => self.jp_c_u16(bus),
            0xDC => self.call_c_u16(bus),
            0xDE => self.sbc_a_fetch_u8(bus),
            0xDF => self.rst(bus, 0x18),
            0xE0 => self.ldh_at_u8_a(bus),
            0xE1 => self.pop_hl(bus),
            0xE2 => self.ldh_at_c_a(bus),
            0xE5 => self.push_hl(bus),
            0xE6 => self.and_a_fetch_u8(bus),
            0xE7 => self.rst(bus, 0x20),
            0xE8 => self.add_sp_i8(bus),
            0xE9 => self.jp_hl(bus),
            0xEA => self.ld_at_u16_a(bus),
            0xEE => self.xor_a_fetch_u8(bus),
            0xEF => self.rst(bus, 0x28),
            0xF0 => self.ldh_a_at_u8(bus),
            0xF1 => self.pop_af(bus),
            0xF2 => self.ldh_a_at_c(bus),
            0xF3 => self.di(),
            0xF5 => self.push_af(bus),
            0xF6 => self.or_a_fetch_u8(bus),
            0xF7 => self.rst(bus, 0x30),
            0xF8 => self.ld_hl_sp_i8(bus),
            0xF9 => self.ld_sp_hl(),
            0xFA => self.ld_a_at_u16(bus),
            0xFB => self.ei(),
            0xFE => self.cp_a_fetch_u8(bus),
            0xFF => self.rst(bus, 0x38),
            _ => panic!(
                "Unimplemented opcode: {:#04X} at pc {:#06X}",
                opcode, self.registers.pc
            ),
        }
    }

    pub fn execute_cb(&mut self, bus: &mut Bus) -> u8 {
        let opcode = self.fetch_u8(bus);
        match opcode {
            0x00 => self.rlc_b(),
            0x01 => self.rlc_c(),
            0x02 => self.rlc_d(),
            0x03 => self.rlc_e(),
            0x04 => self.rlc_h(),
            0x05 => self.rlc_l(),
            0x06 => self.rlc_hl(bus),
            0x07 => self.rlc_a(),
            0x08 => self.rrc_b(),
            0x09 => self.rrc_c(),
            0x0A => self.rrc_d(),
            0x0B => self.rrc_e(),
            0x0C => self.rrc_h(),
            0x0D => self.rrc_l(),
            0x0E => self.rrc_hl(bus),
            0x0F => self.rrc_a(),
            0x10 => self.rl_b(),
            0x11 => self.rl_c(),
            0x12 => self.rl_d(),
            0x13 => self.rl_e(),
            0x14 => self.rl_h(),
            0x15 => self.rl_l(),
            0x16 => self.rl_hl(bus),
            0x17 => self.rl_a(),
            0x18 => self.rr_b(),
            0x19 => self.rr_c(),
            0x1A => self.rr_d(),
            0x1B => self.rr_e(),
            0x1C => self.rr_h(),
            0x1D => self.rr_l(),
            0x1E => self.rr_hl(bus),
            0x1F => self.rr_a(),
            0x20 => self.sla_b(),
            0x21 => self.sla_c(),
            0x22 => self.sla_d(),
            0x23 => self.sla_e(),
            0x24 => self.sla_h(),
            0x25 => self.sla_l(),
            0x26 => self.sla_hl(bus),
            0x27 => self.sla_a(),
            0x28 => self.sra_b(),
            0x29 => self.sra_c(),
            0x2A => self.sra_d(),
            0x2B => self.sra_e(),
            0x2C => self.sra_h(),
            0x2D => self.sra_l(),
            0x2E => self.sra_hl(bus),
            0x2F => self.sra_a(),
            _ => panic!(
                "Unimplemented CB prefix opcode: {:#04X} at pc {:#06X}",
                opcode, self.registers.pc
            ),
        }
    }
}
