pub struct Cartridge {
    rom: Vec<u8>,
}

// todo: make it smarter with MBC and banks... later?
impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        u16::from_le_bytes([self.read_u8(address), self.read_u8(address.wrapping_add(1))])
    }
}
