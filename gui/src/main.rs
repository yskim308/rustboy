use std::{env, fs, io};

use emulator::bus::joypad::JoypadInput;
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
    env_logger::init();
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
        for key in window.get_keys_pressed(minifb::KeyRepeat::No) {
            if let Some(gb_button) = map_key_to_joypad(key) {
                gameboy.press_button(gb_button);
            }
        }

        // Handle newly released keys (Key Up)
        for key in window.get_keys_released() {
            if let Some(gb_button) = map_key_to_joypad(key) {
                gameboy.release_button(gb_button);
            }
        }
        gameboy.step_frame();

        let gb_buffer = gameboy.get_frame_buffer();

        for (i, &pixel_shade) in gb_buffer.iter().enumerate() {
            let color_index = (pixel_shade & 0x03) as usize;
            display_buffer[i] = PALETTE[color_index];
        }

        window
            .update_with_buffer(&display_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn map_key_to_joypad(key: Key) -> Option<JoypadInput> {
    match key {
        Key::J => Some(JoypadInput::A),
        Key::K => Some(JoypadInput::B),
        Key::Space => Some(JoypadInput::Select),
        Key::Enter => Some(JoypadInput::Start),
        Key::D => Some(JoypadInput::Right),
        Key::A => Some(JoypadInput::Left),
        Key::W => Some(JoypadInput::Up),
        Key::S => Some(JoypadInput::Down),
        _ => None,
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
