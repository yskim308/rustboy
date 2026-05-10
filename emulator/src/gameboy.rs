use crate::{bus::Bus, cpu::Cpu};

pub struct Gameboy {
    bus: Bus,
    cpu: Cpu,
}

impl Gameboy {
    pub fn new(bus: Bus, cpu: Cpu) -> Self {
        Self { bus, cpu }
    }

    pub fn step_frame(&mut self) {
        let cycles = self.cpu.step(&mut self.bus);
        self.bus.synchronize(cycles);
    }
}
