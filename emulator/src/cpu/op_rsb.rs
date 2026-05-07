use crate::{bus::Bus, cpu::Cpu};

macro_rules! rlc_r {
    ($func_name: ident, $reg: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            let (shifted, carry) = self.rlc_u8(self.registers.$reg);
            self.registers.$reg = shifted;

            self.set_znhc(shifted == 0, false, false, carry);
            8
        }
    };
}

macro_rules! rrc_r {
    ($func_name: ident, $reg: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            let (shifted, carry) = self.rrc_u8(self.registers.$reg);
            self.registers.$reg = shifted;

            self.set_znhc(shifted == 0, false, false, carry);
            8
        }
    };
}

macro_rules! rl_r {
    ($func_name: ident, $reg: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            let (shifted, carry) = self.rl_u8(self.registers.$reg);
            self.registers.$reg = shifted;

            self.set_znhc(shifted == 0, false, false, carry);
            8
        }
    };
}

macro_rules! rr_r {
    ($func_name: ident, $reg: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            let (shifted, carry) = self.rr_u8(self.registers.$reg);
            self.registers.$reg = shifted;
            self.set_znhc(shifted == 0, false, false, carry);
            8
        }
    };
}

impl Cpu {
    // =========== RR  ==========
    fn rr_u8(&self, mut val: u8) -> (u8, bool) {
        let lsb = val & 1;
        val >>= 1;
        val |= (self.registers.get_c() as u8) << 7;
        (val, lsb == 1)
    }

    rr_r!(rr_b, b);
    rr_r!(rr_c, c);
    rr_r!(rr_d, d);
    rr_r!(rr_e, e);
    rr_r!(rr_h, h);
    rr_r!(rr_l, l);
    rr_r!(rr_a, a);

    pub(super) fn rr_hl(&mut self, bus: &mut Bus) -> u8 {
        let address = self.registers.get_hl();
        let (shifted, carry) = self.rr_u8(bus.read_u8(address));
        bus.write_u8(address, shifted);
        self.set_znhc(shifted == 0, false, false, carry);
        16
    }

    // ========== RL =============
    fn rl_u8(&self, mut val: u8) -> (u8, bool) {
        let msb = (val >> 7) & 1;
        val <<= 1;
        val |= self.registers.get_c() as u8;
        (val, msb == 1)
    }

    rl_r!(rl_b, b);
    rl_r!(rl_c, c);
    rl_r!(rl_d, d);
    rl_r!(rl_e, e);
    rl_r!(rl_h, h);
    rl_r!(rl_l, l);
    rl_r!(rl_a, a);

    pub(super) fn rl_hl(&mut self, bus: &mut Bus) -> u8 {
        let address = self.registers.get_hl();
        let (shifted, carry) = self.rl_u8(bus.read_u8(address));
        bus.write_u8(address, shifted);
        self.set_znhc(shifted == 0, false, false, carry);
        16
    }

    // ============ RRC ==============
    fn rrc_u8(&self, mut val: u8) -> (u8, bool) {
        let lsb = val & 1;
        val = val.rotate_right(1);
        (val, lsb == 1)
    }

    rrc_r!(rrc_b, b);
    rrc_r!(rrc_c, c);
    rrc_r!(rrc_d, d);
    rrc_r!(rrc_e, e);
    rrc_r!(rrc_h, h);
    rrc_r!(rrc_l, l);
    rrc_r!(rrc_a, a);

    pub(super) fn rrc_hl(&mut self, bus: &mut Bus) -> u8 {
        let address = self.registers.get_hl();
        let (shifted, carry) = self.rrc_u8(bus.read_u8(address));
        bus.write_u8(address, shifted);
        self.set_znhc(shifted == 0, false, false, carry);
        16
    }

    // ============== RLC =============
    fn rlc_u8(&self, mut val: u8) -> (u8, bool) {
        let msb = (val >> 7) & 1;
        val = val.rotate_left(1);
        (val, msb == 1)
    }

    rlc_r!(rlc_b, b);
    rlc_r!(rlc_c, c);
    rlc_r!(rlc_d, d);
    rlc_r!(rlc_e, e);
    rlc_r!(rlc_h, h);
    rlc_r!(rlc_l, l);
    rlc_r!(rlc_a, a);

    pub(super) fn rlc_hl(&mut self, bus: &mut Bus) -> u8 {
        let address = self.registers.get_hl();
        let (shifted, carry) = self.rlc_u8(bus.read_u8(address));
        bus.write_u8(address, shifted);
        self.set_znhc(shifted == 0, false, false, carry);
        16
    }

    // =========== accumulatr right/left shift ===================
    fn set_znhc(&mut self, z: bool, n: bool, h: bool, c: bool) {
        self.registers.set_z(z);
        self.registers.set_n(n);
        self.registers.set_h(h);
        self.registers.set_c(c);
    }

    pub(super) fn rlca(&mut self) -> u8 {
        let msb = (self.registers.a >> 7) & 1;
        self.registers.a <<= 1;
        self.registers.a |= msb;

        self.set_znhc(false, false, false, msb == 1);
        4
    }

    pub(super) fn rla(&mut self) -> u8 {
        let msb = (self.registers.a >> 7) & 1;
        self.registers.a <<= 1;
        self.registers.a |= self.registers.get_c() as u8;

        self.set_znhc(false, false, false, msb == 1);
        4
    }

    pub(super) fn rrca(&mut self) -> u8 {
        let lsb = self.registers.a & 1;
        self.registers.a >>= 1;
        self.registers.a |= lsb << 7;

        self.set_znhc(false, false, false, lsb == 1);
        4
    }

    pub(super) fn rra(&mut self) -> u8 {
        let lsb = self.registers.a & 1;
        self.registers.a >>= 1;
        self.registers.a |= (self.registers.get_c() as u8) << 7;

        self.set_znhc(false, false, false, lsb == 1);
        4
    }
}
