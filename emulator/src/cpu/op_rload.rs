use crate::{bus::Bus, cpu::Cpu};

macro_rules! ld_r_u8 {
    ($name:ident, $register:ident) => {
        pub(super) fn $name(&mut self, bus: &mut Bus) -> u8 {
            let value = self.fetch_u8(bus);
            self.registers.$register = value;
            8
        }
    };
}

macro_rules! ld_rr_u16 {
    ($name:ident, $setter:ident) => {
        pub(super) fn $name(&mut self, bus: &mut Bus) -> u8 {
            let value = self.fetch_u16(bus);
            self.registers.$setter(value);
            12
        }
    };
}

macro_rules! ld_r_r {
    ($name:ident, $dst:ident, $src:ident) => {
        pub(super) fn $name(&mut self) -> u8 {
            self.registers.$dst = self.registers.$src;
            4
        }
    };
}

impl Cpu {
    ld_rr_u16!(ld_bc_u16, set_bc);
    ld_rr_u16!(ld_de_u16, set_de);
    ld_rr_u16!(ld_hl_u16, set_hl);

    pub(super) fn ld_sp_u16(&mut self, bus: &mut Bus) -> u8 {
        let value = self.fetch_u16(bus);
        self.registers.sp = value;
        12
    }

    pub(super) fn ld_a_hli(&mut self, bus: &mut Bus) -> u8 {
        let value = bus.read_u8(self.registers.get_hl());
        self.registers.a = value;
        self.registers.set_hl(self.registers.get_hl() + 1);
        8
    }

    ld_r_u8!(ld_b_u8, b);
    ld_r_u8!(ld_c_u8, c);
    ld_r_u8!(ld_d_u8, d);
    ld_r_u8!(ld_e_u8, e);
    ld_r_u8!(ld_h_u8, h);
    ld_r_u8!(ld_l_u8, l);
    ld_r_u8!(ld_a_u8, a);

    ld_r_r!(ld_b_b, b, b);
    ld_r_r!(ld_b_c, b, c);
    ld_r_r!(ld_b_d, b, d);
    ld_r_r!(ld_b_e, b, e);
    ld_r_r!(ld_b_h, b, h);
    ld_r_r!(ld_b_l, b, l);
    ld_r_r!(ld_b_a, b, a);

    ld_r_r!(ld_c_b, c, b);
    ld_r_r!(ld_c_c, c, c);
    ld_r_r!(ld_c_d, c, d);
    ld_r_r!(ld_c_e, c, e);
    ld_r_r!(ld_c_h, c, h);
    ld_r_r!(ld_c_l, c, l);
    ld_r_r!(ld_c_a, c, a);

    ld_r_r!(ld_d_b, d, b);
    ld_r_r!(ld_d_c, d, c);
    ld_r_r!(ld_d_d, d, d);
    ld_r_r!(ld_d_e, d, e);
    ld_r_r!(ld_d_h, d, h);
    ld_r_r!(ld_d_l, d, l);
    ld_r_r!(ld_d_a, d, a);

    ld_r_r!(ld_e_b, e, b);
    ld_r_r!(ld_e_c, e, c);
    ld_r_r!(ld_e_d, e, d);
    ld_r_r!(ld_e_e, e, e);
    ld_r_r!(ld_e_h, e, h);
    ld_r_r!(ld_e_l, e, l);
    ld_r_r!(ld_e_a, e, a);

    ld_r_r!(ld_h_b, h, b);
    ld_r_r!(ld_h_c, h, c);
    ld_r_r!(ld_h_d, h, d);
    ld_r_r!(ld_h_e, h, e);
    ld_r_r!(ld_h_h, h, h);
    ld_r_r!(ld_h_l, h, l);
    ld_r_r!(ld_h_a, h, a);

    ld_r_r!(ld_l_b, l, b);
    ld_r_r!(ld_l_c, l, c);
    ld_r_r!(ld_l_d, l, d);
    ld_r_r!(ld_l_e, l, e);
    ld_r_r!(ld_l_h, l, h);
    ld_r_r!(ld_l_l, l, l);
    ld_r_r!(ld_l_a, l, a);

    ld_r_r!(ld_a_b, a, b);
    ld_r_r!(ld_a_c, a, c);
    ld_r_r!(ld_a_d, a, d);
    ld_r_r!(ld_a_e, a, e);
    ld_r_r!(ld_a_h, a, h);
    ld_r_r!(ld_a_l, a, l);
    ld_r_r!(ld_a_a, a, a);

    pub(super) fn ld_sp_hl(&mut self) -> u8 {
        self.registers.sp = self.registers.get_hl();
        8
    }
}
