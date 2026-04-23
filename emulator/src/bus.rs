use std::panic;

use crate::bus::{cartridge::Cartridge, serial::Serial, wram::Wram};

pub mod cartridge;
pub mod serial;
pub mod wram;

pub struct Bus {
    cartridge: Cartridge, // 0000 - 7FFF
    vram: [u8; 8192],     // 8000 - 9FFF
    eram: [u8; 8192],     // A000 - BFFF
    wram: Wram,           // C000 - DFFF + E000 - FDFF (echo ram)
    oam: [u8; 160],       // FE00 - FE9F

    // IO: FF00 - FF7F
    serial: Serial,       // FF01 - FF02
    io_bucket: [u8; 128], // temporary for io stubs

    hram: [u8; 127], // FF80 - FFFE
    ie_register: u8, // FFFF
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            vram: [0; 8192],
            eram: [0; 8192],
            wram: Wram::new(),
            oam: [0; 160],
            serial: Serial::new(),
            io_bucket: [0; 128],
            hram: [0; 127],
            ie_register: 0,
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_u8(address),
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xA000..=0xBBFF => self.eram[(address - 0xA000) as usize],
            0xC000..=0xDFFF => self.wram.read_u8(address - 0xC000),
            0xE000..=0xFDFF => self.wram.read_u8(address - 0xE000), // echo ram
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFF00..=0xFF7F => self.read_io(address),
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.ie_register,
            _ => panic!("Invalid read access to address {:#04X}", address),
        }
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x7FF => println!("accessed ROM as write for MBC"),
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = data,
            0xA000..=0xBBFF => self.eram[(address - 0xA000) as usize] = data,
            0xC000..=0xDFFF => self.wram.write_u8(address - 0xC000, data),
            0xE000..=0xFDFF => self.wram.write_u8(address - 0xE000, data), // echo ram
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = data,
            0xFF00..=0xFF7F => self.write_io(address, data),
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = data,
            0xFFFF => self.ie_register = data,
            _ => panic!("Invalid write access to address {:#04X}", address),
        }
    }

    fn read_io(&self, address: u16) -> u8 {
        match address {
            0xFF01..=0xFF02 => self.serial.read(address),
            _ => self.io_bucket[(address - 0xFF00) as usize],
        }
    }

    fn write_io(&mut self, address: u16, data: u8) {
        match address {
            0xFF01..=0xFF02 => self.serial.write(address, data),
            _ => self.io_bucket[(address - 0xFF00) as usize] = data,
        }
    }
}
