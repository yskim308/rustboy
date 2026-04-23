use std::io::{self, Write};

pub struct Serial {
    data: u8,
    control: u8,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            data: 0,
            control: 0,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF00 => self.data,
            0xFF01 => self.control,
            _ => unreachable!("Cannot address serial IO outside ranges 0xFF00 - 0xFF01"),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => self.data = value,
            0xFF02 => {
                self.control = value;
                if value == 0x81 {
                    let c = self.data as char;
                    print!("{c}");
                    io::stdout().flush().unwrap(); // unwrap... I assume this would never really fail?
                    self.control &= !0x80;
                }
            }
            _ => unreachable!("Cannot address serial IO outside ranges 0xFF00 - 0xFF01"),
        }
    }
}
