use crate::{bus::Bus, cpu::Cpu};

impl Cpu {
    fn conditional_jump(&mut self, bus: &mut Bus, condition: bool) -> u8 {
        let value = self.fetch_u16(bus);
        if condition {
            self.registers.pc = value;
            16
        } else {
            12
        }
    }

    fn conditional_rel_jump(&mut self, bus: &mut Bus, condition: bool) -> u8 {
        let value = self.fetch_u8(bus) as i8;
        if condition {
            self.registers.pc = self.registers.pc.wrapping_add_signed(value as i16);
            3
        } else {
            2
        }
    }

    fn push_u8(&mut self, bus: &mut Bus, data: u8) {
        self.registers.decrement_sp();
        bus.write_u8(self.registers.sp, data);
    }

    fn push_u16(&mut self, bus: &mut Bus, data: u16) {
        let [low, high] = data.to_le_bytes();
        self.push_u8(bus, high);
        self.push_u8(bus, low);
    }

    fn conditional_call(&mut self, bus: &mut Bus, condition: bool) -> u8 {
        let target_address = self.fetch_u16(bus);
        if condition {
            self.push_u16(bus, self.registers.pc);

            self.registers.pc = target_address;
            24
        } else {
            12
        }
    }

    // ============ CALLS ==================
    pub(super) fn call_nz_u16(&mut self, bus: &mut Bus) -> u8 {
        self.conditional_call(bus, !self.registers.get_z())
    }

    pub(super) fn call_z_u16(&mut self, bus: &mut Bus) -> u8 {
        self.conditional_call(bus, self.registers.get_z())
    }

    pub(super) fn call_u16(&mut self, bus: &mut Bus) -> u8 {
        self.conditional_call(bus, true)
    }

    pub(super) fn call_nc_u16(&mut self, bus: &mut Bus) -> u8 {
        self.conditional_call(bus, !self.registers.get_c())
    }

    pub(super) fn call_c_u16(&mut self, bus: &mut Bus) -> u8 {
        self.conditional_call(bus, self.registers.get_c())
    }

    // ========== RELATIVE JUMPS =============
    pub(super) fn jr_i8(&mut self, bus: &mut Bus) -> u8 {
        self.conditional_rel_jump(bus, true)
    }

    pub(super) fn jr_nz_i8(&mut self, bus: &mut Bus) -> u8 {
        self.conditional_rel_jump(bus, !self.registers.get_z())
    }

    pub(super) fn jr_z_i8(&mut self, bus: &mut Bus) -> u8 {
        self.conditional_rel_jump(bus, self.registers.get_z())
    }

    pub(super) fn jr_nc_i8(&mut self, bus: &mut Bus) -> u8 {
        self.conditional_rel_jump(bus, !self.registers.get_c())
    }

    pub(super) fn jr_c_i8(&mut self, bus: &mut Bus) -> u8 {
        self.conditional_rel_jump(bus, self.registers.get_c())
    }

    // =========== ABSOLUTE JUMPS ==================
    pub(super) fn jp_nz_u16(&mut self, bus: &mut Bus) -> u8 {
        self.conditional_jump(bus, !self.registers.get_z())
    }

    pub(super) fn jp_u16(&mut self, bus: &mut Bus) -> u8 {
        let value = self.fetch_u16(bus);
        self.registers.pc = value;
        16
    }

    pub(super) fn jp_z_u16(&mut self, bus: &mut Bus) -> u8 {
        self.conditional_jump(bus, self.registers.get_z())
    }

    pub(super) fn jp_nc_u16(&mut self, bus: &mut Bus) -> u8 {
        self.conditional_jump(bus, !self.registers.get_c())
    }

    pub(super) fn jp_c_u16(&mut self, bus: &mut Bus) -> u8 {
        self.conditional_jump(bus, self.registers.get_c())
    }

    pub(super) fn jp_hl(&mut self, bus: &mut Bus) -> u8 {
        self.registers.pc = self.registers.get_hl();
        4
    }
}
