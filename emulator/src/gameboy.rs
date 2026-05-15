use crate::{bus::Bus, cpu::Cpu};

const CYCLES_PER_FRAME: u32 = 70_224;

pub struct Gameboy {
    bus: Bus,
    cpu: Cpu,
}

impl Gameboy {
    pub fn new(bus: Bus, cpu: Cpu) -> Self {
        Self { bus, cpu }
    }

    pub fn step_frame(&mut self) {
        let mut frame_cycles = 0;

        while frame_cycles < CYCLES_PER_FRAME {
            let cycles = self.cpu.step(&mut self.bus);
            self.bus.synchronize(cycles);
            frame_cycles += cycles as u32;
        }
    }

    pub fn get_frame_buffer(&self) -> &[u8] {
        self.bus.get_ppu_frame_buffer()
    }
}
