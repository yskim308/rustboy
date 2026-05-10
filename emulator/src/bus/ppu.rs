use crate::bus::Bus;

#[derive(PartialEq)]
enum PpuState {
    OamSearch,
    PixelTransfer,
    HBlank,
    VBlank,
}

struct Sprite {
    x: u8,
    y: u8,
    tile_index: u8,
    flag: u8,
}

#[derive(Default)]
pub struct PpuRegisters {
    pub lcdc: u8,
    pub scx: u8,
    pub scy: u8,
    pub bgp: u8,
}

pub(super) struct Ppu {
    ly: u8,
    cycle: u16,
    state: PpuState,
    registers: PpuRegisters,

    vram: [u8; 8192],
    oam: [u8; 160],

    oam_searched: bool,
    saved_sprites: Vec<Sprite>,
}

impl Ppu {
    pub fn step(&mut self, cycles: u8, ppu_registers: PpuRegisters) {
        self.cycle += cycles as u16;
        self.registers = ppu_registers;
        match self.state {
            PpuState::OamSearch => self.search_oam(self.registers.lcdc),
            PpuState::PixelTransfer => todo!(),
            PpuState::HBlank => todo!(),
            PpuState::VBlank => todo!(),
        }
    }

    fn search_oam(&mut self, lcdc: u8) {
        if self.state != PpuState::OamSearch {
            panic!("Trying to search OAM when PPU State is not OamSearch");
        }

        if !self.oam_searched {
            let obj_size = (lcdc >> 2) & 1;
            let sprite_height = if obj_size == 0 { 8 } else { 16 };
            self.save_sprites(sprite_height);
        }

        if self.cycle > 20 {
            self.state = PpuState::PixelTransfer;
            self.cycle = self.cycle % 20;
            self.oam_searched = false;
        }
    }

    fn save_sprites(&mut self, sprite_height: u8) {
        self.saved_sprites.clear();

        for i in 0..40 {
            if self.saved_sprites.len() >= 10 {
                break;
            }
            let base = i * 4;
            let y = self.oam[base];
            let x = self.oam[base + 1];
            let tile_index = self.oam[base + 2];
            let flag = self.oam[base + 3];

            let ly_offset = self.ly as u16 + 16;
            let is_overlapping =
                y as u16 <= ly_offset && ly_offset <= y as u16 + sprite_height as u16;

            if is_overlapping {
                self.saved_sprites.push(Sprite {
                    x,
                    y,
                    tile_index,
                    flag,
                });
            }
        }
        self.oam_searched = true;
    }

    pub fn read_u8(&self, address: u16) -> u8 {
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
            cycle: 0,
            state: PpuState::OamSearch,
            vram: [0; 8192],
            oam: [0; 160],
            oam_searched: false,
            saved_sprites: Vec::with_capacity(10),
            registers: PpuRegisters::default(),
        }
    }
}
