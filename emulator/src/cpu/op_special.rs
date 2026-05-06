use crate::{bus::Bus, cpu::Cpu};

impl Cpu {
    pub(super) fn di(&mut self) -> u8 {
        self.ime = false;
        self.ime_pending = false;
        4
    }

    pub(super) fn ei(&mut self) -> u8 {
        self.ime_pending = true;
        4
    }

    pub(super) fn halt(&mut self, bus: &mut Bus) -> u8 {
        let pending = bus.read_u8(0xFF0F) & bus.read_u8(0xFFFF) & 0x1F;
        if self.ime {
            self.halt = true;
        } else if pending != 0 {
            self.halt_bug = true;
        } else {
            self.halt = true;
        }
        4
    }

    pub(super) fn stop(&mut self, bus: &mut Bus) -> u8 {
        // just fetch next insruction and do nothing
        self.fetch_u8(bus);
        4
    }

    pub(super) fn nop(&self) -> u8 {
        4
    }
}
