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

pub enum JoypadInput {
    Start,
    Select,
    B,
    A,
    Right,
    Left,
    Up,
    Down,
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

    pub fn press_button(&mut self, button: JoypadInput) -> bool {
        let state_before = self.read_register();

        match button {
            JoypadInput::A => self.a = true,
            JoypadInput::B => self.b = true,
            JoypadInput::Select => self.select = true,
            JoypadInput::Start => self.start = true,
            JoypadInput::Right => self.right = true,
            JoypadInput::Left => self.left = true,
            JoypadInput::Up => self.up = true,
            JoypadInput::Down => self.down = true,
        }

        let state_after = self.read_register();

        let transition = (state_before & !state_after) & 0x0F;

        // trigger interrupt if any transitioned
        transition != 0
    }

    pub fn release_button(&mut self, button: JoypadInput) {
        match button {
            JoypadInput::A => self.a = false,
            JoypadInput::B => self.b = false,
            JoypadInput::Select => self.select = false,
            JoypadInput::Start => self.start = false,
            JoypadInput::Right => self.right = false,
            JoypadInput::Left => self.left = false,
            JoypadInput::Up => self.up = false,
            JoypadInput::Down => self.down = false,
        }
    }

    pub fn write_register(&mut self, data: u8) {
        self.selection_bits = data & 0x30;
    }
}
