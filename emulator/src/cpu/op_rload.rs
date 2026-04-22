use crate::{bus::Bus, cpu::Cpu};

impl Cpu {
    pub(super) fn ld_c_u8(&mut self, bus: &mut Bus) -> u8 {
        let value = self.fetch_u8(bus);
        self.registers.d = value;
        8
    }

    pub(super) fn ld_de_u16(&mut self, bus: &mut Bus) -> u8 {
        let value = self.fetch_u16(bus);
        self.registers.set_de(value);
        12
    }

    pub(super) fn ld_hl_u16(&mut self, bus: &mut Bus) -> u8 {
        let value = self.fetch_u16(bus);
        self.registers.set_hl(value);
        12
    }

    pub(super) fn ld_b_a(&mut self) -> u8 {
        self.registers.b = self.registers.a;
        4
    }
}
