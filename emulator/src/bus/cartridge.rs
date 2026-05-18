pub struct Cartridge {
    rom: Vec<u8>,
    eram: Vec<u8>,
    mbc: Mbc,
}

enum Mbc {
    NoMBC,
    Mbc1(Mbc1Registers),
}

struct Mbc1Registers {
    ram_enable: bool,
    rom_bank_number: u8,
    ram_bank_nubmer: u8,
    banking_mode: u8,
}

impl Mbc1Registers {
    fn new() -> Self {
        Self {
            ram_enable: false,
            rom_bank_number: 1,
            ram_bank_nubmer: 0,
            banking_mode: 0,
        }
    }
}

const KiB: usize = 1024;
// todo: make it smarter with MBC and banks... later?
impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Self {
        let mbc = match rom[0x0147] {
            0x00 => Mbc::NoMBC,
            0x01 => Mbc::Mbc1(Mbc1Registers::new()),
            _ => Mbc::NoMBC,
        };

        let eram_size = match rom[0x0149] {
            0x00 | 0x01 => 0,
            0x02 => 8 * KiB,
            0x03 => 32 * KiB,
            0x04 => 128 * KiB,
            0x05 => 64 * KiB,
            _ => 0,
        };

        Self {
            rom,
            eram: Vec::with_capacity(eram_size),
            mbc,
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        u16::from_le_bytes([self.read_u8(address), self.read_u8(address.wrapping_add(1))])
    }

    pub fn write_u8(&self, address: u16, data: u8) {
        todo!()
    }
}
