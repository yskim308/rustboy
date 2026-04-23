pub struct Wram {
    // later on, make it 32Kib with switchable bank
    data: [u8; 8192],
}

impl Wram {
    pub fn new() -> Self {
        Self { data: [0; 8192] }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write_u8(&mut self, addr: u16, val: u8) {
        self.data[addr as usize] = val;
    }
}
