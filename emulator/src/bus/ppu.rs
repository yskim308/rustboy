#[derive(PartialEq)]
enum PpuState {
    OamSearch,
    PixelTransfer,
    HBlank,
    VBlank,
}

const OAM_SEARCH_CYCLES: u16 = 80;
const PIXEL_TRANSFER_CYCLES: u16 = 172;
const HBLANK_CYCLES: u16 = 204;
const SCANLINE_CYCLES: u16 = 456;

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
    pub obp0: u8,
    pub obp1: u8,
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
    pixel_transferred: bool,

    bg_priority_buffer: [bool; 160],
    bg_palette: [u8; 4],
    obj_palette_0: [u8; 4],
    obj_palette_1: [u8; 4],

    frame_buffer: [u8; 160 * 144],
}

impl Ppu {
    pub fn step(&mut self, cycles: u8, ppu_registers: PpuRegisters) -> bool {
        self.cycle = self.cycle.wrapping_add(cycles as u16);
        self.registers = ppu_registers;
        self.init_palettes();

        if self.registers.lcdc & 0x80 == 0 {
            self.reset_for_disabled_lcd();
            return false;
        }

        match self.state {
            PpuState::OamSearch => self.search_oam(self.registers.lcdc),
            PpuState::PixelTransfer => self.transfer_pixels(),
            PpuState::HBlank => return self.hblank(),
            PpuState::VBlank => return self.vblank(),
        }

        false
    }

    pub fn get_frame_buffer(&self) -> &[u8] {
        &self.frame_buffer
    }

    pub fn ly(&self) -> u8 {
        self.ly
    }

    pub fn reset_ly(&mut self) {
        self.ly = 0;
    }

    pub fn stat(&self, stat_register: u8, lyc: u8) -> u8 {
        let mode = match self.state {
            PpuState::HBlank => 0,
            PpuState::VBlank => 1,
            PpuState::OamSearch => 2,
            PpuState::PixelTransfer => 3,
        };
        let coincidence = u8::from(self.ly == lyc) << 2;

        (stat_register & 0xF8) | coincidence | mode
    }

    // ========================= STATE MACHINE ======================

    fn search_oam(&mut self, lcdc: u8) {
        if self.state != PpuState::OamSearch {
            panic!("Trying to search OAM when PPU State is not OamSearch");
        }

        if !self.oam_searched {
            let obj_size = (lcdc >> 2) & 1;
            let sprite_height = if obj_size == 0 { 8 } else { 16 };
            self.save_sprites(sprite_height);
            self.oam_searched = true;
        }

        if self.cycle >= OAM_SEARCH_CYCLES {
            self.state = PpuState::PixelTransfer;
            self.cycle -= OAM_SEARCH_CYCLES;
            self.oam_searched = false;
        }
    }

    fn transfer_pixels(&mut self) {
        if self.state != PpuState::PixelTransfer {
            panic!("Attempted pixel transfer in non pixel transfer state");
        }

        if !self.pixel_transferred {
            if self.registers.lcdc & 0x01 != 0 {
                self.draw_background();
                self.draw_window();
            } else {
                for i in 0..160 {
                    self.frame_buffer[(self.ly as usize) * 160 + i] = 0;
                }
                self.bg_priority_buffer.fill(false);
            }

            if self.registers.lcdc & 0x02 != 0 {
                self.draw_sprites();
            }
            self.pixel_transferred = true;
        }

        if self.cycle >= PIXEL_TRANSFER_CYCLES {
            self.state = PpuState::HBlank;
            self.cycle -= PIXEL_TRANSFER_CYCLES;
            self.pixel_transferred = false;
        }
    }

    fn hblank(&mut self) -> bool {
        if self.state != PpuState::HBlank {
            panic!("Attempted HBlank in non HBlank state");
        }

        if self.cycle >= HBLANK_CYCLES {
            self.cycle -= HBLANK_CYCLES;

            self.ly += 1;
            if self.ly == 144 {
                self.state = PpuState::VBlank;
                return true;
            } else {
                self.state = PpuState::OamSearch;
            }
        }

        false
    }

    fn vblank(&mut self) -> bool {
        if self.cycle >= SCANLINE_CYCLES {
            self.cycle -= SCANLINE_CYCLES;
            self.ly += 1;

            if self.ly >= 154 {
                self.ly = 0;
                self.state = PpuState::OamSearch;
            }
        }

        false
    }

    fn reset_for_disabled_lcd(&mut self) {
        self.ly = 0;
        self.cycle = 0;
        self.state = PpuState::OamSearch;
        self.oam_searched = false;
        self.pixel_transferred = false;
        self.saved_sprites.clear();
        self.bg_priority_buffer.fill(false);
        self.frame_buffer.fill(0);
    }
    // =========================== HELPERS =========================
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

    fn init_palettes(&mut self) {
        for i in 0..4 {
            self.bg_palette[i] = (self.registers.bgp >> (i * 2)) & 0x03;
            self.obj_palette_0[i] = (self.registers.obp0 >> (i * 2)) & 0x03;
            self.obj_palette_1[i] = (self.registers.obp1 >> (i * 2)) & 0x03;
        }
    }

    // ====================== RENDERING PIPELINE =======================
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
                y as u16 <= ly_offset && ly_offset < y as u16 + sprite_height as u16;

            if is_overlapping {
                self.saved_sprites.push(Sprite {
                    x,
                    y,
                    tile_index,
                    flag,
                });
            }
        }
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
            self.bg_priority_buffer[i as usize] = pixel_val != 0;
            self.frame_buffer[(self.ly as usize * 160) + (i as usize)] =
                self.bg_palette[pixel_val as usize];
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
            self.bg_priority_buffer[i as usize] = pixel_val != 0;
            self.frame_buffer[(self.ly as usize * 160) + (i as usize)] =
                self.bg_palette[pixel_val as usize];
        }
    }

    fn draw_sprites(&mut self) {
        let height = if self.registers.lcdc & 0x04 != 0 {
            16
        } else {
            8
        };

        for sprite in self.saved_sprites.iter().rev() {
            let y_flip = (sprite.flag & 0x40) != 0;
            let x_flip = (sprite.flag & 0x20) != 0;
            let bg_priority = (sprite.flag & 0x80) != 0;
            let is_obp_1 = (sprite.flag & 0x10) != 0;

            let mut tile_row = self.ly + 16 - sprite.y;

            if y_flip {
                tile_row = (height - 1) - tile_row;
            }

            let mut tile_index = sprite.tile_index;
            if height == 16 {
                tile_index &= 0xFE; // ignore bottom bit for 8x16 sprites
                if tile_row >= 8 {
                    tile_index |= 0x01; // use the bottom tile
                }
            }

            let data_address = 0x8000 + (tile_index as u16 * 16);

            for sprite_x in 0..8 {
                let screen_x = sprite.x as i16 - 8 + sprite_x as i16;

                if screen_x < 0 || screen_x >= 160 {
                    continue;
                }

                let fetch_x = if x_flip { 7 - sprite_x } else { sprite_x };
                let pixel_val = self.get_pixel_val(data_address, fetch_x, (tile_row % 8) as u8);
                if pixel_val == 0 {
                    continue;
                }

                let buffer_index = (self.ly as usize * 160) + screen_x as usize;
                if bg_priority && self.bg_priority_buffer[screen_x as usize] {
                    continue;
                }

                let final_shade = if is_obp_1 {
                    self.obj_palette_1[pixel_val as usize]
                } else {
                    self.obj_palette_0[pixel_val as usize]
                };

                self.frame_buffer[buffer_index] = final_shade;
            }
        }
    }

    // ================= READ WRITE TO VRAM/OAM ======================
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
            pixel_transferred: false,
            oam_searched: false,
            saved_sprites: Vec::with_capacity(10),
            bg_priority_buffer: [false; 160],
            bg_palette: [0; 4],
            obj_palette_0: [0; 4],
            obj_palette_1: [0; 4],
            registers: PpuRegisters::default(),
            frame_buffer: [0; 160 * 144],
        }
    }
}
