pub struct Cartridge {
    rom: Vec<u8>,
    eram: Vec<u8>,
    mbc: Mbc,
}

#[derive(PartialEq)]
enum Mbc {
    NoMBC,
    Mbc1(Mbc1),
    Mbc3(Mbc3),
}

#[derive(PartialEq)]
struct Mbc1 {
    ram_enable: bool,
    rom_bank_number: u8,
    ram_bank_number: u8,
    ram_banking_mode: bool,
}

#[derive(PartialEq)]
struct Mbc3 {
    ram_timer_enable: bool,
    rom_bank_number: u8,
    bank_or_register_select: u8,
    latch_ready: bool,
    base_timestamp: u64,
    clock_registers: ClockRegisters,
}

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
    fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_timer_enable = (data & 0x0F) == 0x0A,
            0x2000..=0x3FFF => {
                let bank = 0x7F;
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
}

impl Mbc1 {
    fn new() -> Self {
        Self {
            ram_enable: false,
            rom_bank_number: 1,
            ram_bank_number: 0,
            ram_banking_mode: false,
        }
    }

    fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enable = (data & 0x0F) == 0x0A,
            0x2000..=0x3FFF => {
                let mut rom_bank_number = data & 0x1F;
                if rom_bank_number == 0 {
                    rom_bank_number = 0x01;
                }
                self.rom_bank_number = rom_bank_number;
            }
            0x4000..=0x5FFF => {
                self.ram_bank_number = data & 0x03;
            }
            0x6000..=0x7FFF => {
                self.ram_banking_mode = (data & 0x01) != 0;
            }
            _ => unreachable!("Writing to MBC1 Cartridge in invalid range"),
        }
    }
}

const KiB: usize = 1024;
impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Self {
        let mbc = match rom[0x0147] {
            0x00 => Mbc::NoMBC,
            0x01..=0x03 => Mbc::Mbc1(Mbc1::new()),
            0x0F..=0x13 => Mbc::Mbc3(Mbc3::default()),
            _ => Mbc::NoMBC,
        };

        let eram_size = match rom[0x0149] {
            0x00 | 0x01 => 0,
            0x02 => 8 * KiB,
            0x03 => 32 * KiB,
            0x04 => 128 * KiB,
            0x05 => 64 * KiB,
            _ => 0,
        };

        Self {
            rom,
            eram: vec![0; eram_size],
            mbc,
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.read_bank0(address),
            0x4000..=0x7FFF => self.read_bank1(address),
            0xA000..=0xBFFF => self.read_ram_bank(address),
            _ => 0xFF,
        }
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x7FFF => match &mut self.mbc {
                Mbc::NoMBC => (),
                Mbc::Mbc1(mbc1) => mbc1.write_u8(address, data),
                Mbc::Mbc3(mbc3) => mbc3.write_u8(address, data),
            },
            0xA000..=0xBFFF => self.write_ram(address, data),
            _ => (),
        }
    }

    fn read_bank0(&self, address: u16) -> u8 {
        let bank = match &self.mbc {
            Mbc::NoMBC => 0,
            Mbc::Mbc1(mbc1) => {
                if !mbc1.ram_banking_mode {
                    0
                } else {
                    (mbc1.ram_bank_number << 5) as usize
                }
            }
            Mbc::Mbc3(_) => 0,
        };
        let cartridge_address = (bank * 0x4000) + (address as usize);
        self.rom[cartridge_address % self.rom.len()]
    }

    fn read_bank1(&self, address: u16) -> u8 {
        let cartridge_address = match &self.mbc {
            Mbc::NoMBC => address as usize,
            Mbc::Mbc1(mbc1) => {
                let mut bank = mbc1.rom_bank_number as usize;

                if !mbc1.ram_banking_mode {
                    bank |= (mbc1.ram_bank_number as usize) << 5;
                }

                if bank == 0x00 || bank == 0x20 || bank == 0x40 || bank == 0x60 {
                    bank += 1;
                }

                (bank * 0x4000) + (address - 0x4000) as usize
            }
            Mbc::Mbc3(mbc3) => {
                let bank = mbc3.rom_bank_number as usize;
                (bank * 0x4000) + (address - 0x4000) as usize
            }
        };
        self.rom[cartridge_address % self.rom.len()]
    }

    fn read_ram_bank(&self, address: u16) -> u8 {
        match &self.mbc {
            Mbc::NoMBC => 0xFF,
            Mbc::Mbc1(mbc1) => {
                if !mbc1.ram_enable || self.eram.is_empty() {
                    return 0xFF;
                }
                let bank = if mbc1.ram_banking_mode {
                    mbc1.ram_bank_number as usize
                } else {
                    0 as usize
                };

                let ram_address = (bank * 0x2000) + (address - 0xA000) as usize;
                self.eram[ram_address % self.eram.len()]
            }
            Mbc::Mbc3(mbc3) => {
                if !mbc3.ram_timer_enable {
                    return 0xFF;
                }
                match mbc3.bank_or_register_select {
                    0x00..=0x07 => {
                        if self.eram.is_empty() {
                            return 0xFF;
                        }
                        let ram_address = (mbc3.bank_or_register_select as usize) * 0x2000
                            + (address - 0xA000) as usize;
                        self.eram[ram_address % self.eram.len()]
                    }
                    val @ 0x08..=0x0C => mbc3.clock_registers.get_reg_from_u8(val),
                    _ => 0xFF,
                }
            }
        }
    }

    fn write_ram(&mut self, address: u16, data: u8) {
        match &mut self.mbc {
            Mbc::NoMBC => (),
            Mbc::Mbc1(mbc1) => {
                if !mbc1.ram_enable || self.eram.is_empty() {
                    return;
                }
                let bank = if mbc1.ram_banking_mode {
                    mbc1.ram_bank_number as usize
                } else {
                    0
                };

                let ram_address = (bank * 0x2000) + (address - 0xA000) as usize;
                let len = self.eram.len();
                self.eram[ram_address % len] = data;
            }
            Mbc::Mbc3(mbc3) => {
                if !mbc3.ram_timer_enable {
                    return;
                }
                match mbc3.bank_or_register_select {
                    0x00..=0x07 => {
                        if self.eram.is_empty() {
                            return;
                        }
                        let ram_address = (mbc3.bank_or_register_select as usize) * 0x2000
                            + (address - 0xA000) as usize;
                        let len = self.eram.len();
                        self.eram[ram_address % len] = data
                    }
                    val @ 0x08..=0x0C => mbc3.clock_registers.write_reg_from_u8(val, data),
                    _ => return,
                }
            }
        }
    }
}
