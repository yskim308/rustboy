use crate::{bus::Bus, cpu::Cpu};

macro_rules! add_a_r {
    ($func_name: ident, $source_r: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            self.add_a_u8(self.registers.$source_r);
            4
        }
    };
}

impl Cpu {
    // ============= ADD instructions ==================
    fn add_a_u8(&mut self, val: u8) {
        let source_val = val;
        let (result, carry) = self.registers.a.overflowing_add(source_val);
        let half_carry = (self.registers.a & 0x0F) + (source_val & 0x0F) > 0x0F;
        self.registers.a = result;

        self.registers.set_z(result == 0);
        self.registers.set_n(false);
        self.registers.set_h(half_carry);
        self.registers.set_c(carry);
    }

    add_a_r!(add_a_b, b);
    add_a_r!(add_a_c, c);
    add_a_r!(add_a_d, d);
    add_a_r!(add_a_e, e);
    add_a_r!(add_a_h, h);
    add_a_r!(add_a_l, l);
    add_a_r!(add_a_a, a);

    pub(super) fn add_a_at_hl(&mut self, bus: &mut Bus) -> u8 {
        self.add_a_u8(bus.read_u8(self.registers.get_hl()));
        8
    }
}
