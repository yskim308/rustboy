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

        for (i, &byte) in instruction_bytes.iter().enumerate() {
            rom[0x0100 + i] = byte;
        }

        let bus = Bus::new(Cartridge::new(rom));

        (cpu, bus)
    }

    #[test]
    fn execute_0x0E() {
        // LD C u8, set C to u8, takes 8 cycles
        let (mut cpu, mut bus) = setup(&[0x12]);

        let cycles = cpu.execute(0x0E, &mut bus);
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.c, 0x12);
    }

    #[test]
    fn execute_0x11() {
        // LD DE u16, set DE to u16, takes 12 cycles
        let (mut cpu, mut bus) = setup(&[0x34, 0x12]);

        let cycles = cpu.execute(0x11, &mut bus);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.get_de(), 0x1234);
    }

    #[test]
    fn execute_0x21() {
        // LD HL, u16, sets HL to u16, takes 12 cycles
        let (mut cpu, mut bus) = setup(&[0x34, 0x12]);

        let cycles = cpu.execute(0x21, &mut bus);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.get_hl(), 0x1234);
    }

    #[test]
    fn execute_0x2A() {
        // LD A, (HL+), sets A to mem[HL++], takes 8 cycles
        let (mut cpu, mut bus) = setup(&[0x12]);
        cpu.registers.set_hl(cpu.registers.pc);
        cpu.registers.a = 0;

        let cycles = cpu.execute(0x2A, &mut bus);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.a, 0x12);
        assert_eq!(cpu.registers.get_hl(), 1);
    }

    #[test]
    fn execute_0x47() {
        // LD B, A, sets B to value at A, takes 4 cycles
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.a = 1;
        cpu.registers.b = 5;

        let cycles = cpu.execute(0x47, &mut bus);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.b, 1);
        assert_eq!(cpu.registers.a, 1);
    }

    #[test]
    fn execute_0xC3() {
        // JP u16, sets pc to u16, takes 16 cycles
        let (mut cpu, mut bus) = setup(&[0x34, 0x12]);

        let cycles = cpu.execute(0xC3, &mut bus);

        assert_eq!(cycles, 16);
        assert_eq!(cpu.registers.pc, 0x1234);
    }
}
