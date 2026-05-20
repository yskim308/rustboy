#[derive(PartialEq)]
pub struct Mbc1 {
    ram_enable: bool,
    rom_bank_number: u8,
    ram_bank_number: u8,
    ram_banking_mode: bool,
}

impl Mbc1 {
    pub fn new() -> Self {
        Self {
            ram_enable: false,
            rom_bank_number: 1,
            ram_bank_number: 0,
            ram_banking_mode: false,
        }
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
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

    pub fn read_rom(&self, address: u16, rom: &[u8]) -> u8 {
        match address {
            0x0000..=0x3FFF => {
                let bank = if !self.ram_banking_mode {
                    0
                } else {
                    (self.ram_bank_number << 5) as usize
                };
                let cartridge_address = (bank * 0x4000) + (address as usize);
                rom[cartridge_address % rom.len()]
            }
            0x4000..=0x7FFF => {
                let mut bank = self.rom_bank_number as usize;

                if !self.ram_banking_mode {
                    bank |= (self.ram_bank_number as usize) << 5;
                }

                if bank == 0x00 || bank == 0x20 || bank == 0x40 || bank == 0x60 {
                    bank += 1;
                }

                let cartridge_address = (bank * 0x4000) + (address - 0x4000) as usize;
                rom[cartridge_address % rom.len()]
            }
            _ => 0xFF,
        }
    }

    pub fn read_ram(&self, address: u16, eram: &[u8]) -> u8 {
        if !self.ram_enable || eram.is_empty() {
            return 0xFF;
        }
        let bank = if self.ram_banking_mode {
            self.ram_bank_number as usize
        } else {
            0 as usize
        };

        let ram_address = (bank * 0x2000) + (address - 0xA000) as usize;
        eram[ram_address % eram.len()]
    }

    pub fn write_ram(&mut self, address: u16, data: u8, eram: &mut [u8]) {
        if !self.ram_enable || eram.is_empty() {
            return;
        }
        let bank = if self.ram_banking_mode {
            self.ram_bank_number as usize
        } else {
            0
        };

        let ram_address = (bank * 0x2000) + (address - 0xA000) as usize;
        let len = eram.len();
        eram[ram_address % len] = data;
    }
}
