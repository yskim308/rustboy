pub struct Memory {
    storage: [u8; 0x10000],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            storage: [0; 0x10000],
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        self.storage[address as usize]
    }
    pub fn write_u8(&mut self, address: u16, value: u8) {
        self.storage[address as usize] = value;
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        u16::from_le_bytes([self.read_u8(address), self.read_u8(address.wrapping_add(1))])
    }
    pub fn write_u16(&mut self, address: u16, value: u16) {
        let [lsb, msb] = value.to_le_bytes();
        self.write_u8(address, lsb);
        self.write_u8(address.wrapping_add(1), msb);
    }
}
