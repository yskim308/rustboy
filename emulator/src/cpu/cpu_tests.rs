#[cfg(test)]
mod tests {
    use crate::{
        bus::{Bus, cartridge::Cartridge},
        cpu::Cpu,
    };

    fn setup(instruction_bytes: &[u8]) -> (Cpu, Bus) {
        let mut cpu = Cpu::new();
        cpu.registers.pc = 0x0100;

        let mut rom = vec![0; 0x8000];

        // Write however many bytes you passed in starting at 0x0100
        for (i, &byte) in instruction_bytes.iter().enumerate() {
            rom[0x0100 + i] = byte;
        }

        let bus = Bus::new(Cartridge::new(rom));

        (cpu, bus)
    }

    #[test]
    fn execute_0xC3() {
        // 2. Destructure the tuple to get your cpu and bus instantly
        let (mut cpu, mut bus) = setup(&[0x34, 0x12]);

        let cycles = cpu.execute(0xC3, &mut bus);

        assert_eq!(cycles, 16);
        assert_eq!(cpu.registers.pc, 0x1234);
    }

    #[test]
    fn execute_0x21() {
        let (mut cpu, mut bus) = setup(&[0x34, 0x12]);

        let cycles = cpu.execute(0x21, &mut bus);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.get_hl(), 0x1234);
    }

    #[test]
    fn execute_0x47() {
        // You only need to pass an empty array here since 0x47 is a 1-byte
        // instruction and doesn't read immediate values from the bus!
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.a = 1;
        cpu.registers.b = 5;

        let cycles = cpu.execute(0x47, &mut bus);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.b, 1);
        assert_eq!(cpu.registers.a, 1);
    }
}
