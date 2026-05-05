use crate::{bus::Bus, cpu::Cpu};

macro_rules! inc_r {
    ($func_name: ident, $get_method: ident, $set_method: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            self.registers
                .$set_method(self.registers.$get_method().wrapping_add(1));
            8
        }
    };
}

macro_rules! dec_r {
    ($func_name: ident, $get_method: ident, $set_method: ident) => {
        pub(super) fn $func_name(&mut self) -> u8 {
            self.registers
                .$set_method(self.registers.$get_method().wrapping_sub(1));
            8
        }
    };
}

impl Cpu {
    inc_r!(inc_bc, get_bc, set_bc);
    inc_r!(inc_de, get_de, set_de);
    inc_r!(inc_hl, get_hl, set_hl);

    dec_r!(dec_bc, get_bc, set_bc);
    dec_r!(dec_de, get_de, set_de);
    dec_r!(dec_hl, get_hl, set_hl);
}
