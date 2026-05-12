use std::iter::TakeWhile;

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
    pub wy: u8,
    pub wx: u8,
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

    frame_buffer: [u8; 160 * 140],
}

impl Ppu {
    pub fn step(&mut self, cycles: u8, ppu_registers: PpuRegisters) {
        self.cycle += cycles as u16;
        self.registers = ppu_registers;
        match self.state {
            PpuState::OamSearch => self.search_oam(self.registers.lcdc),
            PpuState::PixelTransfer => self.transfer_pixels(),
            PpuState::HBlank => todo!(),
            PpuState::VBlank => todo!(),
        }
    }

    fn transfer_pixels(&mut self) {
        if self.registers.lcdc & 0x01 != 0 {
            self.draw_background();
            self.draw_window();
        } else {
            for i in 0..160 {
                self.frame_buffer[(self.ly as usize) * 160 + i] = 0;
            }
        }
    }

    fn get_tile_map_address(&self, bit_on: bool) -> u16 {
        if bit_on { 0x9C00 } else { 0x9800 }
    }

    fn get_data_base_address(&self, bit_on: bool) -> u16 {
        if bit_on { 0x8000 } else { 0x8800 }
    }

    fn get_data_address(
        &self,
        tile_row: u16,
        tile_col: u16,
        tile_map_address: u16,
        data_base_address: u16,
    ) -> u16 {
        let tile_address = tile_map_address + tile_row * 32 + tile_col;
        let tile_id = self.vram[(tile_address - 0x8000) as usize];

        let data_address = if data_base_address == 0x8000 {
            data_base_address + (tile_id as u16 * 16)
        } else {
            let signed_id = tile_id as i8;
            (0x9000_i32 + (signed_id as i32 * 16)) as u16
        };
        data_address
    }

    fn get_pixel_val(&self, data_address: u16, x: u8, y: u8) -> u8 {
        let row_offset = ((y % 8) * 2) as u16;
        let data_index = (data_address + row_offset - 0x8000) as usize;

        let low = self.vram[data_index];
        let high = self.vram[data_index.wrapping_add(1)];
        let col_offset = x % 8;
        let bit_pos = 7 - col_offset;

        let color_bit_0 = (low >> bit_pos) & 1;
        let color_bit_1 = (high >> bit_pos) & 1;

        (color_bit_1 << 1) | color_bit_0
    }

    fn draw_window(&mut self) {
        if (self.registers.lcdc & 0x20) == 0 || self.ly < self.registers.wy {
            return;
        }

        let bit_6_on = (self.registers.lcdc & 0x40) != 0;
        let tile_map_address = self.get_tile_map_address(bit_6_on);

        let bit_4_on = (self.registers.lcdc & 0x10) != 0;
        let data_base_address = self.get_data_base_address(bit_4_on);

        let window_y = self.ly - self.registers.wy;
        let tile_row = (window_y / 8) as u16;

        let screen_start_x = if self.registers.wx < 7 {
            0
        } else {
            self.registers.wx - 7
        };

        for i in screen_start_x..160 {
            let window_x = i + 7 - self.registers.wx;
            let tile_col = (window_x / 8) as u16;
            let data_address =
                self.get_data_address(tile_row, tile_col, tile_map_address, data_base_address);
            let pixel_val = self.get_pixel_val(data_address, window_x, window_y);
            self.frame_buffer[(self.ly as usize * 160) + (i as usize)] = pixel_val;
        }
    }

    fn draw_background(&mut self) {
        let bit_3_on = (self.registers.lcdc & 0x08) != 0;
        let tile_map_address = self.get_tile_map_address(bit_3_on);

        let bit_4_on = (self.registers.lcdc & 0x10) != 0;
        let data_base_address = self.get_data_base_address(bit_4_on);

        let y = self.ly.wrapping_add(self.registers.scy);
        let tile_row = (y / 8) as u16;
        for i in 0..160 {
            let x = self.registers.scx.wrapping_add(i);

            let tile_col = (x / 8) as u16;
            let data_address =
                self.get_data_address(tile_row, tile_col, tile_map_address, data_base_address);
            let pixel_val = self.get_pixel_val(data_address, x, y);
            self.frame_buffer[(self.ly as usize * 160) + (i as usize)] = pixel_val;
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
            frame_buffer: [0; 160 * 140],
        }
    }
}
