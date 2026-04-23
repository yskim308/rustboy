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
                eprintln!("Memory address {address} not implemented yet in Bus");
                todo!("handle")
            }
        }
    }

    fn read_io(&self, address: u16) -> u8 {
        match address {
            0xFF01..=0xFF02 => self.serial.read(address),
            _ => self.io_bucket[address as usize],
        }
    }
}
