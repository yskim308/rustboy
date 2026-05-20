#[derive(PartialEq, Default)]
struct ClockRegisters {
    seconds: u8,
    minutes: u8,
    hours: u8,
    day_low: u8,
    day_high: u8,
}

impl ClockRegisters {
    fn get_reg_from_u8(&self, reg_val: u8) -> u8 {
        match reg_val {
            0x08 => self.seconds,
            0x09 => self.minutes,
            0x0A => self.hours,
            0x0B => self.day_low,
            0x0C => self.day_high,
            _ => unreachable!(),
        }
    }

    fn write_reg_from_u8(&mut self, reg_val: u8, val: u8) {
        match reg_val {
            0x08 => self.seconds = val,
            0x09 => self.minutes = val,
            0x0A => self.hours = val,
            0x0B => self.day_low = val,
            0x0C => self.day_high = val,
            _ => (),
        }
    }
}

#[derive(PartialEq)]
pub struct Mbc3 {
    ram_timer_enable: bool,
    rom_bank_number: u8,
    bank_or_register_select: u8,
    latch_ready: bool,
    base_timestamp: u64,
    clock_registers: ClockRegisters,
}

impl Default for Mbc3 {
    fn default() -> Self {
        Self {
            ram_timer_enable: false,
            rom_bank_number: 0,
            bank_or_register_select: 0,
            latch_ready: false,
            base_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            clock_registers: ClockRegisters::default(),
        }
    }
}

impl Mbc3 {
    pub fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_timer_enable = (data & 0x0F) == 0x0A,
            0x2000..=0x3FFF => {
                let bank = data & 0x7F;
                self.rom_bank_number = if bank == 0 { 1 } else { bank }
            }
            0x4000..=0x5FFF => self.bank_or_register_select = data,
            0x6000..=0x7FFF => {
                if data == 0x00 {
                    self.latch_ready = true;
                } else if data == 0x01 {
                    if self.latch_ready {
                        self.latch_clock();
                    }
                    self.latch_ready = false;
                } else {
                    self.latch_ready = false;
                }
            }
            _ => (),
        }
    }

    fn latch_clock(&mut self) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let elapsed = current_time.saturating_sub(self.base_timestamp);

        self.clock_registers.seconds = (elapsed % 60) as u8;
        self.clock_registers.minutes = ((elapsed / 60) % 60) as u8;
        self.clock_registers.hours = ((elapsed / 3600) % 24) as u8;

        let days = elapsed / 86400;

        if days > 511 {
            self.clock_registers.day_high |= 0x80;
        }

        let clamped_days = days % 512;

        self.clock_registers.day_low = (clamped_days & 0xFF) as u8;

        let bit_9 = (clamped_days >> 8) as u8;
        self.clock_registers.day_high = (self.clock_registers.day_high & 0xFE) | bit_9;
    }

    pub fn read_rom(&self, address: u16, rom: &[u8]) -> u8 {
        match address {
            0x0000..=0x3FFF => rom[address as usize % rom.len()],
            0x4000..=0x7FFF => {
                let bank = self.rom_bank_number as usize;
                let cartridge_address = (bank * 0x4000) + (address - 0x4000) as usize;
                rom[cartridge_address % rom.len()]
            }
            _ => 0xFF,
        }
    }

    pub fn read_ram(&self, address: u16, eram: &[u8]) -> u8 {
        if !self.ram_timer_enable {
            return 0xFF;
        }
        match self.bank_or_register_select {
            0x00..=0x07 => {
                if eram.is_empty() {
                    return 0xFF;
                }
                let ram_address = (self.bank_or_register_select as usize) * 0x2000
                    + (address - 0xA000) as usize;
                eram[ram_address % eram.len()]
            }
            val @ 0x08..=0x0C => self.clock_registers.get_reg_from_u8(val),
            _ => 0xFF,
        }
    }

    pub fn write_ram(&mut self, address: u16, data: u8, eram: &mut [u8]) {
        if !self.ram_timer_enable {
            return;
        }
        match self.bank_or_register_select {
            0x00..=0x07 => {
                if eram.is_empty() {
                    return;
                }
                let ram_address = (self.bank_or_register_select as usize) * 0x2000
                    + (address - 0xA000) as usize;
                let len = eram.len();
                eram[ram_address % len] = data;
            }
            val @ 0x08..=0x0C => self.clock_registers.write_reg_from_u8(val, data),
            _ => return,
        }
    }
}
