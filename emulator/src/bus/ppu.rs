enum PpuState {
    OamSearch,
    PixelTransfer,
    HBlank,
    VBlank,
}

pub(super) struct Ppu {
    ly: u8,
    scx: u8,
    scy: u8,
    cycle: u64,
    state: PpuState,

    vram: [u8; 8192],
    oam: [u8; 160],
}

impl Ppu {
    pub fn step(&mut self, cycles: u8) {
        self.cycle += cycles as u64;
        todo!("handling PPU state machine")
    }

    pub fn read_u8(&mut self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            _ => unreachable!("PPU read access ({address}) outside of vram and oam range"),
        }
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = data,
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = data,
            _ => unreachable!("PPU write access ({address}) outside of vram and oam range"),
        }
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Ppu {
            ly: 0,
            scx: 0,
            scy: 0,
            cycle: 0,
            state: PpuState::OamSearch,
            vram: [0; 8192],
            oam: [0; 160],
        }
    }
}
