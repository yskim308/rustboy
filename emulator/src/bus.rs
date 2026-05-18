use crate::bus::{
    cartridge::Cartridge, joypad::Joypad, ppu::Ppu, serial::Serial, timer::Timer, wram::Wram,
};

pub mod cartridge;
pub mod joypad;
mod ppu;
pub mod serial;
mod timer;
pub mod wram;

pub struct Bus {
    cartridge: Cartridge, // 0000 - 7FFF
    eram: [u8; 8192],     // A000 - BFFF
    wram: Wram,           // C000 - DFFF + E000 - FDFF (echo ram)

    // IO: FF00 - FF7F
    serial: Serial,       // FF01 - FF02
    io_bucket: [u8; 128], // temporary for io stubs

    hram: [u8; 127], // FF80 - FFFE
    ie_register: u8, // FFFF

    ppu: Ppu,
    timer: Timer,
    pub joypad: Joypad,
}

#[repr(u8)]
pub enum InterruptType {
    VBlank = 0,
    LcdStat = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        let mut io_bucket = [0; 128];
        set_post_boot_io_defaults(&mut io_bucket);

        Self {
            cartridge,
            wram: Wram::new(),
            serial: Serial::new(),
            io_bucket,
            hram: [0; 127],
            ie_register: 0,
            ppu: Ppu::new(),
            timer: Timer::new(),
            joypad: Joypad::default(),
        }
    }

    pub fn synchronize(&mut self, cycles: u8) {
        let (request_vblank_interrupt, request_stat_interrupt) = self.ppu.step(cycles);
        let request_timer_interrupt = self.timer.step(cycles);

        if request_vblank_interrupt {
            self.request_interrupt(InterruptType::VBlank);
        }

        if request_stat_interrupt {
            self.request_interrupt(InterruptType::LcdStat);
        }

        if request_timer_interrupt {
            self.request_interrupt(InterruptType::Timer);
        }
    }

    pub fn get_ppu_frame_buffer(&self) -> &[u8] {
        self.ppu.get_frame_buffer()
    }

    pub fn request_interrupt(&mut self, interrupt: InterruptType) {
        self.io_bucket[(0xFF0F - 0xFF00) as usize] |= 1 << (interrupt as u8);
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.read_u8(address),
            0x8000..=0x9FFF => self.ppu.read_u8(address),
            0xC000..=0xDFFF => self.wram.read_u8(address - 0xC000),
            0xE000..=0xFDFF => self.wram.read_u8(address - 0xE000), // echo ram
            0xFE00..=0xFE9F => self.ppu.read_u8(address),
            0xFEA0..=0xFEFF => 0x00,
            0xFF00..=0xFF7F => self.read_io(address),
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.ie_register,
        }
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.write_u8(address, data),
            0x8000..=0x9FFF => self.ppu.write_u8(address, data),
            0xC000..=0xDFFF => self.wram.write_u8(address - 0xC000, data),
            0xE000..=0xFDFF => self.wram.write_u8(address - 0xE000, data), // echo ram
            0xFE00..=0xFE9F => self.ppu.write_u8(address, data),
            0xFEA0..=0xFEFF => (),
            0xFF00..=0xFF7F => self.write_io(address, data),
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = data,
            0xFFFF => self.ie_register = data,
        }
    }

    fn read_io(&self, address: u16) -> u8 {
        match address {
            0xFF00 => self.joypad.read_register(),
            0xFF01..=0xFF02 => self.serial.read(address),
            0xFF04..=0xFF07 => self.timer.read_u8(address),
            0xFF40..=0xFF45 | 0xFF47..=0xFF4B => self.ppu.read_io(address),
            _ => self.io_bucket[(address - 0xFF00) as usize],
        }
    }

    fn write_io(&mut self, address: u16, data: u8) {
        match address {
            0xFF00 => self.joypad.write_register(data),
            0xFF01..=0xFF02 => self.serial.write(address, data),
            0xFF04..=0xFF07 => self.timer.write_u8(address, data),
            0xFF46 => self.oam_dma_transfer(data),
            0xFF40..=0xFF45 | 0xFF47..=0xFF4B => self.ppu.write_io(address, data),
            _ => self.io_bucket[(address - 0xFF00) as usize] = data,
        }
    }

    fn oam_dma_transfer(&mut self, data: u8) {
        let base_address = (data as u16) << 8;

        for offset in 0..160u16 {
            let value = self.read_u8(base_address + offset);
            self.ppu.write_u8(0xFE00 + offset, value);
        }

        self.io_bucket[(0xFF46 - 0xFF00) as usize] = data;
    }
}

fn set_post_boot_io_defaults(io_bucket: &mut [u8; 128]) {
    let defaults = [
        (0xFF05, 0x00),
        (0xFF06, 0x00),
        (0xFF07, 0x00),
        (0xFF10, 0x80),
        (0xFF11, 0xBF),
        (0xFF12, 0xF3),
        (0xFF14, 0xBF),
        (0xFF16, 0x3F),
        (0xFF17, 0x00),
        (0xFF19, 0xBF),
        (0xFF1A, 0x7F),
        (0xFF1B, 0xFF),
        (0xFF1C, 0x9F),
        (0xFF1E, 0xBF),
        (0xFF20, 0xFF),
        (0xFF21, 0x00),
        (0xFF22, 0x00),
        (0xFF23, 0xBF),
        (0xFF24, 0x77),
        (0xFF25, 0xF3),
        (0xFF26, 0xF1),
        (0xFF0F, 0xE1),
    ];

    for (address, value) in defaults {
        io_bucket[(address - 0xFF00) as usize] = value;
    }
}
