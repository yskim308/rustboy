use crate::bus::cartridge;

pub struct Cartridge {
    rom: Vec<u8>,
    eram: Vec<u8>,
    mbc: Mbc,
}

#[derive(PartialEq)]
enum Mbc {
    NoMBC,
    Mbc1(Mbc1),
}

#[derive(PartialEq)]
struct Mbc1 {
    ram_enable: bool,
    rom_bank_number: u8,
    ram_bank_number: u8,
    ram_banking_mode: bool,
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

    pub fn read_u16(&self, address: u16) -> u16 {
        u16::from_le_bytes([self.read_u8(address), self.read_u8(address.wrapping_add(1))])
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x7FFF => match &mut self.mbc {
                Mbc::NoMBC => (),
                Mbc::Mbc1(mbc1) => mbc1.write_u8(address, data),
            },
            0xA000..=0xBFFF => self.write_ram(address, data),
            _ => (),
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
        }
    }
}
