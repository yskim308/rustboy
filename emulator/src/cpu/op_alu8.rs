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

impl Cpu {
    // ============= ADD/ADC instructions ==================
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
}
