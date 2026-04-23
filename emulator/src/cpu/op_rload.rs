use crate::{bus::Bus, cpu::Cpu};

impl Cpu {
    // 0x0E: LD C u8, set C to u8, takes 8 cycles
    pub(super) fn ld_c_u8(&mut self, bus: &mut Bus) -> u8 {
        let value = self.fetch_u8(bus);
        self.registers.c = value;
        8
    }
    // 0x11: LD DE u16, set DE to u16, takes 12 cycles
    pub(super) fn ld_de_u16(&mut self, bus: &mut Bus) -> u8 {
        let value = self.fetch_u16(bus);
        self.registers.set_de(value);
        12
    }
    // 0x21: LD HL, u16, sets HL to u16, takes 12 cycles
    pub(super) fn ld_hl_u16(&mut self, bus: &mut Bus) -> u8 {
        let value = self.fetch_u16(bus);
        self.registers.set_hl(value);
        12
    }
    // 0x2A: LD A, (HL+), sets A to mem[HL++], takes 8 cycles
    pub(super) fn ld_a_hli(&mut self, bus: &mut Bus) -> u8 {
        let value = bus.read_u8(self.registers.get_hl());
        self.registers.a = value;
        self.registers.set_hl(self.registers.get_hl() + 1);
        8
    }

    // 0x40: LD B, B
    pub(super) fn ld_b_b(&mut self) -> u8 {
        4
    }

    // 0x41: LD B, C
    pub(super) fn ld_b_c(&mut self) -> u8 {
        self.registers.b = self.registers.c;
        4
    }

    // 0x47:
    pub(super) fn ld_b_a(&mut self) -> u8 {
        self.registers.b = self.registers.a;
        4
    }
}
