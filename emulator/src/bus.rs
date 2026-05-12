use std::panic;

use crate::bus::{
    cartridge::Cartridge,
    ppu::{Ppu, PpuRegisters},
    serial::Serial,
    wram::Wram,
};

pub mod cartridge;
mod ppu;
pub mod serial;
pub mod wram;

pub struct Bus {
    cartridge: Cartridge, // 0000 - 7FFF
    eram: [u8; 8192],     // A000 - BFFF
    wram: Wram,           // C000 - DFFF + E000 - FDFF (echo ram)

    // IO: FF00 - FF7F
    serial: Serial,       // FF01 - FF02
    io_bucket: [u8; 128], // temporary for io stubs

    hram: [u8; 127], // FF80 - FFFE
    ie_register: u8, // FFFF

    ppu: Ppu,
}

impl Bus {
    pub fn synchronize(&mut self, cycles: u8) {
        let ppu_registers = PpuRegisters {
            lcdc: self.read_u8(0xFF40),
            scx: self.read_u8(0xFF42),
            scy: self.read_u8(0xFF43),
            wy: self.read_u8(0xFF4A),
            wx: self.read_u8(0xFF4B),
            bgp: self.read_u8(0xFF47),
        };
        self.ppu.step(cycles, ppu_registers);
    }

    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            eram: [0; 8192],
            wram: Wram::new(),
            serial: Serial::new(),
            io_bucket: [0; 128],
            hram: [0; 127],
            ie_register: 0,
            ppu: Ppu::default(),
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_u8(address),
            0x8000..=0x9FFF => self.ppu.read_u8(address),
            0xA000..=0xBBFF => self.eram[(address - 0xA000) as usize],
            0xC000..=0xDFFF => self.wram.read_u8(address - 0xC000),
            0xE000..=0xFDFF => self.wram.read_u8(address - 0xE000), // echo ram
            0xFE00..=0xFE9F => self.ppu.read_u8(address),
            0xFF00..=0xFF7F => self.read_io(address),
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.ie_register,
            _ => panic!("Invalid read access to address {:#04X}", address),
        }
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x7FF => println!("accessed ROM as write for MBC"),
            0x8000..=0x9FFF => self.ppu.write_u8(address, data),
            0xA000..=0xBBFF => self.eram[(address - 0xA000) as usize] = data,
            0xC000..=0xDFFF => self.wram.write_u8(address - 0xC000, data),
            0xE000..=0xFDFF => self.wram.write_u8(address - 0xE000, data), // echo ram
            0xFE00..=0xFE9F => self.ppu.write_u8(address, data),
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
