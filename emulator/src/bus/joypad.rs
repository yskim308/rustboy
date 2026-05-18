#[derive(Default)]
pub struct Joypad {
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,

    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,

    selection_bits: u8,
}

impl Joypad {
    pub fn read_register(&self) -> u8 {
        let mut lower_nibble = 0x0F;

        if (self.selection_bits & 0x20) == 0 {
            if self.a {
                lower_nibble &= !(1 << 0);
            }
            if self.b {
                lower_nibble &= !(1 << 1);
            }
            if self.select {
                lower_nibble &= !(1 << 2);
            }
            if self.start {
                lower_nibble &= !(1 << 3);
            }
        }

        if (self.selection_bits & 0x10) == 0 {
            if self.right {
                lower_nibble &= !(1 << 0);
            }
            if self.left {
                lower_nibble &= !(1 << 1);
            }
            if self.up {
                lower_nibble &= !(1 << 2);
            }
            if self.down {
                lower_nibble &= !(1 << 3);
            }
        }

        0xC0 | self.selection_bits | lower_nibble
    }

    pub fn write_register(&mut self, data: u8) {
        self.selection_bits = data & 0x30;
    }
}
