use crate::{bus::Bus, cpu::Cpu};

pub struct Gameboy {
    bus: Bus,
    cpu: Cpu,
}

impl Gameboy {
    pub fn new(bus: Bus, cpu: Cpu) -> Self {
        Self { bus, cpu }
    }

    pub fn step_frame(&mut self) -> u8 {
        self.cpu.step(&mut self.bus)
    }

    pub fn synchronize(&mut self) {
        todo!("Implemnt synch with other components")
    }
}
