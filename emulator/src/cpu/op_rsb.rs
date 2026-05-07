use crate::{bus::Bus, cpu::Cpu};

impl Cpu {
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
