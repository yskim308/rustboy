use crate::{bus::Bus, cpu::Cpu};

macro_rules! ld_reg_at_hl {
    ($func_name: ident, $reg: ident) => {
        pub(super) fn $func_name(&mut self, bus: &mut Bus) -> u8 {
            self.registers.$reg = bus.read_u8(self.registers.get_hl());
            8
        }
    };
}

macro_rules! ld_at_hl_reg {
    ($func_name: ident, $reg: ident) => {
        pub(super) fn $func_name(&mut self, bus: &mut Bus) -> u8 {
            bus.write_u8(self.registers.get_hl(), self.registers.$reg);
            8
        }
    };
}

impl Cpu {
    pub(super) fn ld_at_bc_a(&mut self, bus: &mut Bus) -> u8 {
        bus.write_u8(self.registers.get_bc(), self.registers.a);
        8
    }

    pub(super) fn ld_a_at_bc(&mut self, bus: &mut Bus) -> u8 {
        self.registers.a = bus.read_u8(self.registers.get_bc());
        8
    }

    pub(super) fn ld_at_de_a(&mut self, bus: &mut Bus) -> u8 {
        bus.write_u8(self.registers.get_de(), self.registers.a);
        8
    }

    pub(super) fn ld_a_at_de(&mut self, bus: &mut Bus) -> u8 {
        self.registers.a = bus.read_u8(self.registers.get_de());
        8
    }

    pub(super) fn ld_at_hli_a(&mut self, bus: &mut Bus) -> u8 {
        bus.write_u8(self.registers.get_hl(), self.registers.a);
        self.registers
            .set_hl(self.registers.get_hl().wrapping_add(1));
        8
    }

    pub(super) fn ld_at_hld_a(&mut self, bus: &mut Bus) -> u8 {
        bus.write_u8(self.registers.get_hl(), self.registers.a);
        self.registers
            .set_hl(self.registers.get_hl().wrapping_sub(1));
        8
    }

    pub(super) fn ld_at_hl_u8(&mut self, bus: &mut Bus) -> u8 {
        let value = self.fetch_u8(bus);
        bus.write_u8(self.registers.get_hl(), value);
        12
    }

    pub(super) fn ld_a_at_hld(&mut self, bus: &mut Bus) -> u8 {
        self.registers.a = bus.read_u8(self.registers.get_hl());
        self.registers
            .set_hl(self.registers.get_hl().wrapping_sub(1));
        8
    }

    ld_reg_at_hl!(ld_a_at_hl, a);
    ld_reg_at_hl!(ld_b_at_hl, b);
    ld_reg_at_hl!(ld_c_at_hl, c);
    ld_reg_at_hl!(ld_d_at_hl, d);
    ld_reg_at_hl!(ld_e_at_hl, e);
    ld_reg_at_hl!(ld_h_at_hl, h);
    ld_reg_at_hl!(ld_l_at_hl, l);

    ld_at_hl_reg!(ld_at_hl_a, a);
    ld_at_hl_reg!(ld_at_hl_b, b);
    ld_at_hl_reg!(ld_at_hl_c, c);
    ld_at_hl_reg!(ld_at_hl_d, d);
    ld_at_hl_reg!(ld_at_hl_e, e);
    ld_at_hl_reg!(ld_at_hl_h, h);
    ld_at_hl_reg!(ld_at_hl_l, l);

    pub(super) fn ld_at_u16_a(&mut self, bus: &mut Bus) -> u8 {
        let value = self.fetch_u16(bus);
        bus.write_u8(value, self.registers.a);
        16
    }

    pub(super) fn ld_a_at_u16(&mut self, bus: &mut Bus) -> u8 {
        let address = self.fetch_u16(bus);
        let value = bus.read_u8(address);
        self.registers.a = value;
        16
    }

    // ============== built in vector (0xFF00 + ...) opcodes =============
    pub(super) fn ldh_at_u8_a(&mut self, bus: &mut Bus) -> u8 {
        let offset = self.fetch_u8(bus);
        bus.write_u8(0xFF00 + offset as u16, self.registers.a);
        12
    }

    pub(super) fn ldh_a_at_u8(&mut self, bus: &mut Bus) -> u8 {
        let offset = self.fetch_u8(bus);
        let value = bus.read_u8(0xFF00 + offset as u16);
        self.registers.a = value;
        12
    }

    pub(super) fn ldh_at_c_a(&mut self, bus: &mut Bus) -> u8 {
        bus.write_u8(0xFF00 + self.registers.c as u16, self.registers.a);
        8
    }

    pub(super) fn ldh_a_at_c(&mut self, bus: &mut Bus) -> u8 {
        self.registers.a = bus.read_u8(0xFF00 + self.registers.c as u16);
        8
    }
}
