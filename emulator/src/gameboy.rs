use crate::{bus::Bus, cpu::Cpu};

pub struct Gameboy {
    bus: Bus,
    cpu: Cpu,
}

impl Gameboy {
    pub fn new(bus: Bus, cpu: Cpu) -> Self {
        Self { bus, cpu }
    }
}
