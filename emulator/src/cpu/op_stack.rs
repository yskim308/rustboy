use crate::{bus::Bus, cpu::Cpu};

macro_rules! pop_r16 {
    ($func_name: ident, $set_method: ident) => {
        pub(super) fn $func_name(&mut self, bus: &mut Bus) -> u8 {
            let value = self.pop_u16(bus);
            self.registers.$set_method(value);
            12
        }
    };
}

macro_rules! push_r16 {
    ($func_name: ident, $get_method: ident) => {
        pub(super) fn $func_name(&mut self, bus: &mut Bus) -> u8 {
            self.push_u16(bus, self.registers.$get_method());
            16
        }
    };
}

impl Cpu {
    pop_r16!(pop_bc, set_bc);
    pop_r16!(pop_de, set_de);
    pop_r16!(pop_hl, set_hl);
    pop_r16!(pop_af, set_af);

    push_r16!(push_bc, get_bc);
    push_r16!(push_de, get_de);
    push_r16!(push_hl, get_hl);
    push_r16!(push_af, get_af);
}
