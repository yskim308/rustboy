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
            0xFF01 => self.data,
            0xFF02 => self.control,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_control_without_start() {
        let mut serial = Serial::new();

        // Write 0x01 (External Clock, but NO Start Transfer flag)
        serial.write(0xFF02, 0x01);

        assert_eq!(
            serial.read(0xFF02),
            0x01,
            "Control register should store the value"
        );
    }

    #[test]
    fn test_transfer_clears_start_bit() {
        let mut serial = Serial::new();

        // 1. Load data
        serial.write(0xFF01, 0x41);

        // 2. Trigger transfer (Start flag 0x80 + Internal Clock 0x01 = 0x81)
        serial.write(0xFF02, 0x81);

        // 3. Verify the hardware automatically cleared the 0x80 bit
        // If it started as 0x81, clearing 0x80 leaves us with 0x01.
        assert_eq!(
            serial.read(0xFF02),
            0x01,
            "Top bit (0x80) should be cleared after transfer completes"
        );
    }

    #[test]
    #[should_panic(expected = "Cannot address serial IO outside ranges")]
    fn test_out_of_bounds_write_panics() {
        let mut serial = Serial::new();
        serial.write(0xFF00, 0x00);
    }
}
