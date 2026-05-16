pub(super) struct Timer {
    pub div: u16,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    tima_cycle_counter: u16,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0xABCC,
            tima: 0,
            tma: 0,
            tac: 0xF8,
            tima_cycle_counter: 0,
        }
    }

    pub fn step(&mut self, cycles: u8) -> bool {
        let mut request_interrupt = false;

        self.div = self.div.wrapping_add(cycles as u16);

        if (self.tac & 0x04) != 0 {
            self.tima_cycle_counter += cycles as u16;
            let threshold = match self.tac & 0x03 {
                0b00 => 1024,
                0b01 => 16,
                0b10 => 64,
                0b11 => 256,
                _ => unreachable!(),
            };

            while self.tima_cycle_counter >= threshold {
                self.tima_cycle_counter -= threshold;
                let (new_time, overflow) = self.tima.overflowing_add(1);
                if overflow {
                    self.tima = self.tma;
                    request_interrupt = true;
                } else {
                    self.tima = new_time;
                }
            }
        }

        request_interrupt
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => unreachable!(),
        }
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = data,
            0xFF06 => self.tma = data,
            0xFF07 => self.tac = data | 0xF8,
            _ => unreachable!(),
        }
    }
}
