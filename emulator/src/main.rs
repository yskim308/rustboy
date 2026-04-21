use std::{fs, io};

use crate::{
    bus::{
        Bus,
        cartridge::{self, Cartridge},
    },
    cpu::Cpu,
    gameboy::Gameboy,
};

mod bus;
mod cpu;
mod gameboy;

fn main() {
    let mut file_path = String::new();

    println!("Path (path/to/rom): ");
    io::stdin()
        .read_line(&mut file_path)
        .expect_err("Failed while reading input: ");

    let rom_data = fs::read(file_path).expect("Failed to read ROM from given file path");

    let cartridge = Cartridge::new(rom_data);
    let mut bus = Bus::new(cartridge);
    let cpu = Cpu::new();
    let gameboy = Gameboy::new(bus, cpu);
}
