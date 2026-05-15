use std::{env, fs, io};

use emulator::{Bus, Cartridge, Cpu, Gameboy};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

const PALETTE: [u32; 4] = [
    0xFF_FFFFFF, // 0: White
    0xFF_AAAAAA, // 1: Light Gray
    0xFF_555555, // 2: Dark Gray
    0xFF_000000, // 3: Black
];

fn main() {
    let file_path = env::args().nth(1).unwrap_or_else(read_file_path);

    let rom_data = fs::read(file_path).expect("Failed to read ROM from given file path");

    let cartridge = Cartridge::new(rom_data);
    let bus = Bus::new(cartridge);
    let cpu = Cpu::new();
    let mut gameboy = Gameboy::new(bus, cpu);

    let mut display_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "RustBoy Emulator",
        WIDTH * 2, // Scale up by 2x so it's not tiny
        HEIGHT * 2,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to 60 FPS (Game Boy native speed is ~59.7 FPS)
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        gameboy.step_frame();

        let gb_buffer = gameboy.get_frame_buffer();

        // 3. Translate the u8 shades into u32 ARGB colors
        for (i, &pixel_shade) in gb_buffer.iter().enumerate() {
            // Guard against out-of-bounds if your palette data ever exceeds 3
            let color_index = (pixel_shade & 0x03) as usize;
            display_buffer[i] = PALETTE[color_index];
        }

        // 4. Update the window
        window
            .update_with_buffer(&display_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn read_file_path() -> String {
    let mut file_path = String::new();

    println!("Path (path/to/rom): ");
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed while reading input");

    file_path.trim().to_string()
}
