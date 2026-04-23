use crate::bus::{cartridge::Cartridge, wram::Wram};

pub mod cartridge;
pub mod wram;

pub struct Bus {
    cartridge: Cartridge,
    wram: Wram,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            wram: Wram::new(),
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_u8(address),
            0xC000..=0xDFFF => self.wram.read_u8(address - 0xC000),
            0xE000..=0xFDFF => self.wram.read_u8(address - 0xE000), // echo ram
            _ => {
                eprintln!("Memory address {address} not implemented yet in Bus");
                todo!("handle")
            }
        }
    }
}
