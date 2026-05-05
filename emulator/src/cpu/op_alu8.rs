use crate::{bus::Bus, cpu::Cpu};

macro_rules! add_a_r {
    ($func_name: ident, $source_r: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            self.add_a_u8(self.registers.$source_r, false);
            4
        }
    };
}

macro_rules! adc_a_r {
    ($func_name: ident, $source_r: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            self.add_a_u8(self.registers.$source_r, self.registers.get_c());
            4
        }
    };
}

macro_rules! sub_a_r {
    ($func_name: ident, $source_r: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            self.sub_a_u8(self.registers.$source_r, false);
            4
        }
    };
}

macro_rules! sbc_a_r {
    ($func_name: ident, $source_r: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            self.sub_a_u8(self.registers.$source_r, self.registers.get_c());
            4
        }
    };
}

macro_rules! and_a_r {
    ($func_name: ident, $source_r: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            self.and_a_u8(self.registers.$source_r);
            4
        }
    };
}

macro_rules! xor_a_r {
    ($func_name: ident, $source_r: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            self.xor_a_u8(self.registers.$source_r);
            4
        }
    };
}

macro_rules! or_a_r {
    ($func_name: ident, $source_r: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            self.or_a_u8(self.registers.$source_r);
            4
        }
    };
}

macro_rules! cp_a_r {
    ($func_name: ident, $source_r: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            self.cp_a_u8(self.registers.$source_r);
            4
        }
    };
}

macro_rules! inc_r {
    ($func_name: ident, $dest_r: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            self.registers.$dest_r = self.inc_val(self.registers.$dest_r);
            4
        }
    };
}

macro_rules! dec_r {
    ($func_name: ident, $dest_r: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            self.registers.$dest_r = self.dec_val(self.registers.$dest_r);
            4
        }
    };
}

impl Cpu {
    // ============= helpers =============
    fn dec_val(&mut self, val: u8) -> u8 {
        let result = val.wrapping_sub(1);
        let half_carry = (val & 0x0F) == 0x00;

        self.registers.set_z(result == 0);
        self.registers.set_n(true);
        self.registers.set_h(half_carry);

        result
    }

    fn inc_val(&mut self, val: u8) -> u8 {
        let result = val.wrapping_add(1);

        let half_carry = (val & 0x0F) == 0x0F;

        self.registers.set_z(result == 0);
        self.registers.set_n(false);
        self.registers.set_h(half_carry);

        result
    }

    fn or_a_u8(&mut self, val: u8) {
        self.registers.a |= val;

        self.registers.set_z(self.registers.a == 0);
        self.registers.set_n(false);
        self.registers.set_h(false);
        self.registers.set_c(false);
    }

    fn xor_a_u8(&mut self, val: u8) {
        self.registers.a ^= val;

        self.registers.set_z(self.registers.a == 0);
        self.registers.set_n(false);
        self.registers.set_h(false);
        self.registers.set_c(false);
    }

    fn add_a_u8(&mut self, val: u8, is_carry: bool) {
        let a = self.registers.a as u16;
        let source_val = val as u16;
        let carry_in = is_carry as u16;

        let result = a + source_val + carry_in;
        let half_carry = (a & 0x0F) + (source_val & 0x0F) + carry_in > 0x0F;
        let carry_out = result > 0xFF;

        let result = result as u8;
        self.registers.a = result;

        self.registers.set_z(result == 0);
        self.registers.set_n(false);
        self.registers.set_h(half_carry);
        self.registers.set_c(carry_out);
    }

    fn sub_a_u8(&mut self, val: u8, is_carry: bool) {
        let a = self.registers.a as u16;
        let source_val = val as u16;
        let carry_in = is_carry as u16;

        let result = a.wrapping_sub(source_val).wrapping_sub(carry_in);
        let half_carry = (a & 0x0F) < (source_val & 0x0F) + carry_in;
        let carry_out = a < source_val + carry_in;

        let result = result as u8;
        self.registers.a = result;

        self.registers.set_z(result == 0);
        self.registers.set_n(true);
        self.registers.set_h(half_carry);
        self.registers.set_c(carry_out);
    }

    fn cp_a_u8(&mut self, val: u8) {
        let a = self.registers.a as u16;
        let source_val = val as u16;

        let result = a.wrapping_sub(source_val);
        let half_carry = (a & 0x0F) < (source_val & 0x0F);
        let carry_out = a < source_val;

        self.registers.set_z(result as u8 == 0);
        self.registers.set_n(true);
        self.registers.set_h(half_carry);
        self.registers.set_c(carry_out);
    }

    fn and_a_u8(&mut self, val: u8) {
        self.registers.a &= val;

        self.registers.set_z(self.registers.a == 0);
        self.registers.set_n(false);
        self.registers.set_h(true);
        self.registers.set_c(false);
    }

    // ============= ADD/ADC instructions ==================
    add_a_r!(add_a_b, b);
    add_a_r!(add_a_c, c);
    add_a_r!(add_a_d, d);
    add_a_r!(add_a_e, e);
    add_a_r!(add_a_h, h);
    add_a_r!(add_a_l, l);
    add_a_r!(add_a_a, a);

    pub(super) fn add_a_at_hl(&mut self, bus: &mut Bus) -> u8 {
        let val_at_hl = bus.read_u8(self.registers.get_hl());
        self.add_a_u8(val_at_hl, false);
        8
    }

    pub(super) fn add_a_fetch_u8(&mut self, bus: &mut Bus) -> u8 {
        let value = self.fetch_u8(bus);
        self.add_a_u8(value, false);
        8
    }

    adc_a_r!(adc_a_b, b);
    adc_a_r!(adc_a_c, c);
    adc_a_r!(adc_a_d, d);
    adc_a_r!(adc_a_e, e);
    adc_a_r!(adc_a_h, h);
    adc_a_r!(adc_a_l, l);
    adc_a_r!(adc_a_a, a);

    pub(super) fn adc_a_at_hl(&mut self, bus: &mut Bus) -> u8 {
        let val_at_hl = bus.read_u8(self.registers.get_hl());
        self.add_a_u8(val_at_hl, self.registers.get_c());
        8
    }

    pub(super) fn adc_a_fetch_u8(&mut self, bus: &mut Bus) -> u8 {
        let value = self.fetch_u8(bus);
        self.add_a_u8(value, self.registers.get_c());
        8
    }

    // ============== SUB/SBC ===============
    sub_a_r!(sub_a_b, b);
    sub_a_r!(sub_a_c, c);
    sub_a_r!(sub_a_d, d);
    sub_a_r!(sub_a_e, e);
    sub_a_r!(sub_a_h, h);
    sub_a_r!(sub_a_l, l);
    sub_a_r!(sub_a_a, a);

    pub(super) fn sub_a_at_hl(&mut self, bus: &mut Bus) -> u8 {
        let val_at_hl = bus.read_u8(self.registers.get_hl());
        self.sub_a_u8(val_at_hl, false);
        8
    }

    pub(super) fn sub_a_fetch_u8(&mut self, bus: &mut Bus) -> u8 {
        let val = self.fetch_u8(bus);
        self.sub_a_u8(val, false);
        8
    }

    sbc_a_r!(sbc_a_b, b);
    sbc_a_r!(sbc_a_c, c);
    sbc_a_r!(sbc_a_d, d);
    sbc_a_r!(sbc_a_e, e);
    sbc_a_r!(sbc_a_h, h);
    sbc_a_r!(sbc_a_l, l);
    sbc_a_r!(sbc_a_a, a);

    pub(super) fn sbc_a_at_hl(&mut self, bus: &mut Bus) -> u8 {
        let val_at_hl = bus.read_u8(self.registers.get_hl());
        self.sub_a_u8(val_at_hl, self.registers.get_c());
        8
    }

    pub(super) fn sbc_a_fetch_u8(&mut self, bus: &mut Bus) -> u8 {
        let val = self.fetch_u8(bus);
        self.sub_a_u8(val, self.registers.get_c());
        8
    }

    // ============ AND ============
    and_a_r!(and_a_b, b);
    and_a_r!(and_a_c, c);
    and_a_r!(and_a_d, d);
    and_a_r!(and_a_e, e);
    and_a_r!(and_a_h, h);
    and_a_r!(and_a_l, l);
    and_a_r!(and_a_a, a);

    pub(super) fn and_a_at_hl(&mut self, bus: &mut Bus) -> u8 {
        let val = bus.read_u8(self.registers.get_hl());
        self.and_a_u8(val);
        8
    }

    pub(super) fn and_a_fetch_u8(&mut self, bus: &mut Bus) -> u8 {
        let val = self.fetch_u8(bus);
        self.and_a_u8(val);
        8
    }

    // ========================== XOR ========================
    xor_a_r!(xor_a_b, b);
    xor_a_r!(xor_a_c, c);
    xor_a_r!(xor_a_d, d);
    xor_a_r!(xor_a_e, e);
    xor_a_r!(xor_a_h, h);
    xor_a_r!(xor_a_l, l);
    xor_a_r!(xor_a_a, a);

    pub(super) fn xor_a_at_hl(&mut self, bus: &mut Bus) -> u8 {
        let val = bus.read_u8(self.registers.get_hl());
        self.xor_a_u8(val);
        8
    }

    pub(super) fn xor_a_fetch_u8(&mut self, bus: &mut Bus) -> u8 {
        let val = self.fetch_u8(bus);
        self.xor_a_u8(val);
        8
    }

    // ====================== OR ==========================
    or_a_r!(or_a_b, b);
    or_a_r!(or_a_c, c);
    or_a_r!(or_a_d, d);
    or_a_r!(or_a_e, e);
    or_a_r!(or_a_h, h);
    or_a_r!(or_a_l, l);
    or_a_r!(or_a_a, a);

    pub(super) fn or_a_at_hl(&mut self, bus: &mut Bus) -> u8 {
        let val = bus.read_u8(self.registers.get_hl());
        self.or_a_u8(val);
        8
    }

    pub(super) fn or_a_fetch_u8(&mut self, bus: &mut Bus) -> u8 {
        let val = self.fetch_u8(bus);
        self.or_a_u8(val);
        8
    }

    // ======================= CP =======================
    cp_a_r!(cp_a_b, b);
    cp_a_r!(cp_a_c, c);
    cp_a_r!(cp_a_d, d);
    cp_a_r!(cp_a_e, e);
    cp_a_r!(cp_a_h, h);
    cp_a_r!(cp_a_l, l);
    cp_a_r!(cp_a_a, a);

    pub(super) fn cp_a_at_hl(&mut self, bus: &mut Bus) -> u8 {
        let val = bus.read_u8(self.registers.get_hl());
        self.cp_a_u8(val);
        8
    }

    pub(super) fn cp_a_fetch_u8(&mut self, bus: &mut Bus) -> u8 {
        let val = self.fetch_u8(bus);
        self.cp_a_u8(val);
        8
    }

    // ==================== INC ===================
    inc_r!(inc_b, b);
    inc_r!(inc_c, c);
    inc_r!(inc_d, d);
    inc_r!(inc_e, e);
    inc_r!(inc_h, h);
    inc_r!(inc_l, l);
    inc_r!(inc_a, a);

    pub(super) fn inc_at_hl(&mut self, bus: &mut Bus) -> u8 {
        let value = bus.read_u8(self.registers.get_hl());
        bus.write_u8(self.registers.get_hl(), self.inc_val(value));
        12
    }

    // ================== DEC =============
    dec_r!(dec_b, b);
    dec_r!(dec_c, c);
    dec_r!(dec_d, d);
    dec_r!(dec_e, e);
    dec_r!(dec_h, h);
    dec_r!(dec_l, l);
    dec_r!(dec_a, a);

    pub(super) fn dec_at_hl(&mut self, bus: &mut Bus) -> u8 {
        let value = bus.read_u8(self.registers.get_hl());
        bus.write_u8(self.registers.get_hl(), self.dec_val(value));
        12
    }

    // ==================== special for A register =================
    pub(super) fn daa(&mut self) -> u8 {
        if !self.registers.get_n() {
            if self.registers.a & 0x0F > 9 || self.registers.get_h() {
                self.registers.a = self.registers.a.wrapping_add(0x06);
            }

            if self.registers.a > 0x99 || self.registers.get_c() {
                self.registers.a = self.registers.a.wrapping_add(0x60);
                self.registers.set_c(true);
            }
        } else {
            if self.registers.get_h() {
                self.registers.a = self.registers.a.wrapping_sub(0x06);
            }

            if self.registers.get_c() {
                self.registers.a = self.registers.a.wrapping_sub(0x60);
            }
        }

        self.registers.set_z(self.registers.a == 0);
        self.registers.set_h(false);
        4
    }

    pub(super) fn scf(&mut self) -> u8 {
        self.registers.set_n(false);
        self.registers.set_h(false);
        self.registers.set_c(true);
        4
    }

    pub(super) fn cpl(&mut self) -> u8 {
        self.registers.a = !self.registers.a;
        self.registers.set_n(true);
        self.registers.set_h(true);
        4
    }

    pub(super) fn ccf(&mut self) -> u8 {
        self.registers.set_n(false);
        self.registers.set_h(false);

        if self.registers.get_c() {
            self.registers.set_c(false);
        } else {
            self.registers.set_c(true);
        }

        4
    }
}
