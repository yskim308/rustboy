use std::{env, fs, io};

use crate::{
    bus::{Bus, cartridge::Cartridge, wram::Wram},
    cpu::Cpu,
    gameboy::Gameboy,
};

mod bus;
mod cpu;
mod gameboy;

fn main() {
    let file_path = env::args().nth(1).unwrap_or_else(read_file_path);

    let rom_data = fs::read(file_path).expect("Failed to read ROM from given file path");

    let cartridge = Cartridge::new(rom_data);
    let bus = Bus::new(cartridge);
    let cpu = Cpu::new();
    let mut gameboy = Gameboy::new(bus, cpu);
    loop {
        gameboy.step_frame();
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
