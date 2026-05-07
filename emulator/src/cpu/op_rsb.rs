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

macro_rules! sla_r {
    ($func_name: ident, $reg: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            let (shifted, carry) = self.sla_u8(self.registers.$reg);
            self.registers.$reg = shifted;
            self.set_znhc(shifted == 0, false, false, carry);
            8
        }
    };
}

macro_rules! sra_r {
    ($func_name: ident, $reg: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            let (shifted, carry) = self.sra_u8(self.registers.$reg);
            self.registers.$reg = shifted;
            self.set_znhc(shifted == 0, false, false, carry);
            8
        }
    };
}

macro_rules! swap_r {
    ($func_name: ident, $reg: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            let swapped = self.swap_u8(self.registers.$reg);
            self.registers.$reg = swapped;
            self.set_znhc(swapped == 0, false, false, false);
            8
        }
    };
}

macro_rules! srl_r {
    ($func_name: ident, $reg: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            let (shifted, carry) = self.srl_u8(self.registers.$reg);
            self.registers.$reg = shifted;
            self.set_znhc(shifted == 0, false, false, carry);
            8
        }
    };
}

impl Cpu {
    // ============ SRL =============
    fn srl_u8(&self, mut val: u8) -> (u8, bool) {
        let lsb = val & 1;
        val >>= 1;
        (val, lsb == 1)
    }

    srl_r!(srl_b, b);
    srl_r!(srl_c, c);
    srl_r!(srl_d, d);
    srl_r!(srl_e, e);
    srl_r!(srl_h, h);
    srl_r!(srl_l, l);
    srl_r!(srl_a, a);

    pub(super) fn srl_hl(&mut self, bus: &mut Bus) -> u8 {
        let address = self.registers.get_hl();
        let (shifted, carry) = self.srl_u8(bus.read_u8(address));
        bus.write_u8(address, shifted);
        self.set_znhc(shifted == 0, false, false, carry);
        16
    }

    // =========== SWAP ==========
    fn swap_u8(&self, mut val: u8) -> u8 {
        val = (val << 4) | (val >> 4);
        val
    }

    swap_r!(swap_b, b);
    swap_r!(swap_c, c);
    swap_r!(swap_d, d);
    swap_r!(swap_e, e);
    swap_r!(swap_h, h);
    swap_r!(swap_l, l);
    swap_r!(swap_a, a);

    pub(super) fn swap_hl(&mut self, bus: &mut Bus) -> u8 {
        let address = self.registers.get_hl();
        let swapped = self.swap_u8(bus.read_u8(address));
        bus.write_u8(address, swapped);
        self.set_znhc(swapped == 0, false, false, false);
        16
    }

    // ============ SRA =========
    fn sra_u8(&self, mut val: u8) -> (u8, bool) {
        let msb = (val >> 7) & 1;
        let lsb = val & 1;
        val >>= 1;
        val |= msb << 7;
        (val, lsb == 1)
    }

    sra_r!(sra_b, b);
    sra_r!(sra_c, c);
    sra_r!(sra_d, d);
    sra_r!(sra_e, e);
    sra_r!(sra_h, h);
    sra_r!(sra_l, l);
    sra_r!(sra_a, a);

    pub(super) fn sra_hl(&mut self, bus: &mut Bus) -> u8 {
        let address = self.registers.get_hl();
        let (shifted, carry) = self.sra_u8(bus.read_u8(address));
        bus.write_u8(address, shifted);
        self.set_znhc(shifted == 0, false, false, carry);
        16
    }

    // =========== SLA ==========
    fn sla_u8(&self, mut val: u8) -> (u8, bool) {
        let msb = (val >> 7) & 1;
        val <<= 1;
        (val, msb == 1)
    }

    sla_r!(sla_b, b);
    sla_r!(sla_c, c);
    sla_r!(sla_d, d);
    sla_r!(sla_e, e);
    sla_r!(sla_h, h);
    sla_r!(sla_l, l);
    sla_r!(sla_a, a);

    pub(super) fn sla_hl(&mut self, bus: &mut Bus) -> u8 {
        let address = self.registers.get_hl();
        let (shifted, carry) = self.sla_u8(bus.read_u8(address));
        bus.write_u8(address, shifted);
        self.set_znhc(shifted == 0, false, false, carry);
        16
    }

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
