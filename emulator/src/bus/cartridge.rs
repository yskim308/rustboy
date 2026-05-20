pub mod mbc1;
pub mod mbc3;

use mbc1::Mbc1;
use mbc3::Mbc3;

pub struct Cartridge {
    rom: Vec<u8>,
    eram: Vec<u8>,
    mbc: Mbc,
}

#[derive(PartialEq)]
pub enum Mbc {
    NoMBC,
    Mbc1(Mbc1),
    Mbc3(Mbc3),
}

impl Mbc {
    pub fn read_rom(&self, address: u16, rom: &[u8]) -> u8 {
        match self {
            Mbc::NoMBC => rom[address as usize % rom.len()],
            Mbc::Mbc1(mbc1) => mbc1.read_rom(address, rom),
            Mbc::Mbc3(mbc3) => mbc3.read_rom(address, rom),
        }
    }

    pub fn write_rom(&mut self, address: u16, data: u8) {
        match self {
            Mbc::NoMBC => (),
            Mbc::Mbc1(mbc1) => mbc1.write_u8(address, data),
            Mbc::Mbc3(mbc3) => mbc3.write_u8(address, data),
        }
    }

    pub fn read_ram(&self, address: u16, eram: &[u8]) -> u8 {
        match self {
            Mbc::NoMBC => 0xFF,
            Mbc::Mbc1(mbc1) => mbc1.read_ram(address, eram),
            Mbc::Mbc3(mbc3) => mbc3.read_ram(address, eram),
        }
    }

    pub fn write_ram(&mut self, address: u16, data: u8, eram: &mut [u8]) {
        match self {
            Mbc::NoMBC => (),
            Mbc::Mbc1(mbc1) => mbc1.write_ram(address, data, eram),
            Mbc::Mbc3(mbc3) => mbc3.write_ram(address, data, eram),
        }
    }
}

const KIB: usize = 1024;

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
            0x02 => 8 * KIB,
            0x03 => 32 * KIB,
            0x04 => 128 * KIB,
            0x05 => 64 * KIB,
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
            0x0000..=0x7FFF => self.mbc.read_rom(address, &self.rom),
            0xA000..=0xBFFF => self.mbc.read_ram(address, &self.eram),
            _ => 0xFF,
        }
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x7FFF => self.mbc.write_rom(address, data),
            0xA000..=0xBFFF => self.mbc.write_ram(address, data, &mut self.eram),
            _ => (),
        }
    }
}
