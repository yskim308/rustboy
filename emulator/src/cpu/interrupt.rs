use crate::{bus::Bus, cpu::Cpu};

impl Cpu {
    pub(super) fn handle_interrupts(&mut self, bus: &mut Bus) -> u8 {
        let enable = bus.read_u8(0xFFFF);
        let flag = bus.read_u8(0xFF0F);
        let pending = enable & flag & 0x1F;

        if pending == 0 {
            return 0;
        }

        self.halt = false;

        if !self.ime {
            return 0;
        }

        self.ime = false;
        let priority_bit = pending.trailing_zeros();

        let reset_if = flag & !(1 << priority_bit);
        bus.write_u8(0xFF0F, reset_if);

        self.push_u16(bus, self.registers.pc);

        self.registers.pc = match priority_bit {
            0 => 0x0040,
            1 => 0x0048,
            2 => 0x0050,
            3 => 0x0058,
            4 => 0x0060,
            _ => unreachable!("Bit index must be 0-4 due to 0x1F mask and != 0 check"),
        };

        20
    }
}
