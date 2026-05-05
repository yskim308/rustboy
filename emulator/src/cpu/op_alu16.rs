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
    fn add_hl_rr(&mut self, val: u16) {
        let hl = self.registers.get_hl();

        let result = hl.wrapping_add(val);

        let half_carry = (hl & 0x0FFF) + (val & 0x0FFF) > 0x0FFF;

        let carry_out = (hl as u32) + (val as u32) > 0xFFFF; // Or use overflowing_add

        self.registers.set_hl(result);
        self.registers.set_n(false);
        self.registers.set_h(half_carry);
        self.registers.set_c(carry_out);
    }

    pub(super) fn add_hl_bc(&mut self) -> u8 {
        self.add_hl_rr(self.registers.get_bc());
        8
    }

    pub(super) fn add_hl_de(&mut self) -> u8 {
        self.add_hl_rr(self.registers.get_de());
        8
    }

    pub(super) fn add_hl_hl(&mut self) -> u8 {
        self.add_hl_rr(self.registers.get_hl());
        8
    }

    pub(super) fn add_hl_sp(&mut self) -> u8 {
        self.add_hl_rr(self.registers.sp);
        8
    }

    pub(super) fn add_sp_i8(&mut self, bus: &mut Bus) -> u8 {
        let val = self.fetch_u8(bus);
        let sp = self.registers.sp;

        let offset = val as i8 as i16 as u16;
        let result = sp.wrapping_add(offset);

        // calculate the flags using the UNSIGNED 8-bit representation
        let half_carry = (sp & 0x0F) + (val as u16 & 0x0F) > 0x0F;
        let carry_out = (sp & 0xFF) + (val as u16) > 0xFF;

        self.registers.sp = result;

        self.registers.set_z(false);
        self.registers.set_n(false);
        self.registers.set_h(half_carry);
        self.registers.set_c(carry_out);

        16
    }

    pub(super) fn ld_hl_sp_i8(&mut self, bus: &mut Bus) -> u8 {
        let val = self.fetch_u8(bus);
        let sp = self.registers.sp;

        let offset = val as i8 as i16 as u16;
        let result = sp.wrapping_add(offset);

        let half_carry = (sp & 0x0F) + (val as u16 & 0x0F) > 0x0F;
        let carry_out = (sp & 0xFF) + (val as u16) > 0xFF;

        self.registers.set_hl(result);

        self.registers.set_z(false);
        self.registers.set_n(false);
        self.registers.set_h(half_carry);
        self.registers.set_c(carry_out);

        12
    }

    inc_r!(inc_bc, get_bc, set_bc);
    inc_r!(inc_de, get_de, set_de);
    inc_r!(inc_hl, get_hl, set_hl);

    dec_r!(dec_bc, get_bc, set_bc);
    dec_r!(dec_de, get_de, set_de);
    dec_r!(dec_hl, get_hl, set_hl);
}
