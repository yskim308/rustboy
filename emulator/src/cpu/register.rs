pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    f: u8, // we'll protect the flag register
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    // todo: double check with docs to see if these initial values are correct
    pub fn new() -> Self {
        Self {
            a: 0x01,
            f: 0xB0, // Z, H, C flags set
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100, // Entry point for the ROM
        }
    }
    // Flag register where [z, n, h, c] are upper four bits
    // setting flag registers (rustfmt ain't letting me inline these :/)
    fn set_flag(&mut self, mask: u8, val: bool) {
        if val { self.f |= mask } else { self.f &= !mask }
    }
    pub fn set_z(&mut self, val: bool) {
        self.set_flag(0x80, val);
    }
    pub fn set_n(&mut self, val: bool) {
        self.set_flag(0x40, val);
    }
    pub fn set_h(&mut self, val: bool) {
        self.set_flag(0x20, val);
    }
    pub fn set_c(&mut self, val: bool) {
        self.set_flag(0x10, val);
    }
    pub fn get_f(&mut self) -> u8 {
        self.f
    }

    // getting flag registers
    pub fn get_z(&self) -> bool {
        (self.f & 0x80) != 0
    }
    pub fn get_n(&self) -> bool {
        (self.f & 0x40) != 0
    }
    pub fn get_h(&self) -> bool {
        (self.f & 0x20) != 0
    }
    pub fn get_c(&self) -> bool {
        (self.f & 0x10) != 0
    }

    // [AF] = [A high][F low]
    pub fn get_af(&self) -> u16 {
        u16::from_be_bytes([self.a, self.f])
    }
    pub fn set_af(&mut self, val: u16) {
        let bytes = val.to_be_bytes();
        self.a = bytes[0];
        self.f = bytes[1] & 0xF0; // force lower bits to be 0
    }

    // [BC] = [B High][B Low]
    pub fn get_bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }
    pub fn set_bc(&mut self, val: u16) {
        let bytes = val.to_be_bytes();
        self.b = bytes[0];
        self.c = bytes[1];
    }

    // [DE] = [D High][E Low]
    pub fn get_de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }
    pub fn set_de(&mut self, val: u16) {
        let bytes = val.to_be_bytes();
        self.d = bytes[0];
        self.e = bytes[1];
    }

    // [HL] = [H high][L low]
    pub fn get_hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }
    pub fn set_hl(&mut self, val: u16) {
        let bytes = val.to_be_bytes();
        self.h = bytes[0];
        self.l = bytes[1];
    }
}
