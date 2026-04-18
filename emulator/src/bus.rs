use crate::bus::cartridge::Cartridge;

pub mod cartridge;

pub struct Bus {
    cartridge: Cartridge,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self { cartridge }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_u8(address),
            _ => todo!("other bus things"),
        }
    }
}
