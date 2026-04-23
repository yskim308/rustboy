use crate::bus::{cartridge::Cartridge, serial::Serial, wram::Wram};

pub mod cartridge;
pub mod serial;
pub mod wram;

pub struct Bus {
    cartridge: Cartridge,
    wram: Wram,
    serial: Serial,
    io_bucket: [u8; 128], // temporary for io stubs
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            wram: Wram::new(),
            serial: Serial::new(),
            io_bucket: [0; 128],
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_u8(address),
            0xC000..=0xDFFF => self.wram.read_u8(address - 0xC000),
            0xE000..=0xFDFF => self.wram.read_u8(address - 0xE000), // echo ram
            0xFF00..=0xFF70 => self.read_io(address),
            _ => {
                eprintln!("Memory address {:#04X} not implemented yet in Bus", address);
                todo!("Finish memory map for reads")
            }
        }
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x7FF => todo!("implement writing to ROM for MBC control"),
            0xC000..=0xDFFF => self.wram.write_u8(address - 0xC000, data),
            0xE000..=0xFDFF => self.wram.write_u8(address - 0xE000, data), // echo ram
            0xFF00..=0xFF70 => self.write_io(address, data),
            _ => {
                eprintln!("Memory address {:#04X} not implemented yet in Bus", address);
                todo!("Finish memory map for writes")
            }
        }
    }

    fn read_io(&self, address: u16) -> u8 {
        match address {
            0xFF01..=0xFF02 => self.serial.read(address),
            _ => self.io_bucket[address as usize],
        }
    }

    fn write_io(&mut self, address: u16, data: u8) {
        match address {
            0xFF01..=0xFF02 => self.serial.write(address, data),
            _ => self.io_bucket[address as usize] = data,
        }
    }
}
