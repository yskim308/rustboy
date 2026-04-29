#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use crate::{
        bus::{Bus, cartridge::Cartridge},
        cpu::Cpu,
    };

    const MEM_ADDR: u16 = 0xC123;
    const IO_ADDR: u16 = 0xFF42;

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

    macro_rules! test_ld_at_pair_a {
        ($name:ident, $opcode:expr, $setter:ident) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[]);
                cpu.registers.$setter(MEM_ADDR);
                cpu.registers.a = 0x12;

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 8);
                assert_eq!(bus.read_u8(MEM_ADDR), 0x12);
                assert_eq!(cpu.registers.a, 0x12);
            }
        };
    }

    macro_rules! test_ld_a_at_pair {
        ($name:ident, $opcode:expr, $setter:ident) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[]);
                cpu.registers.$setter(MEM_ADDR);
                bus.write_u8(MEM_ADDR, 0x12);
                cpu.registers.a = 0x00;

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 8);
                assert_eq!(cpu.registers.a, 0x12);
            }
        };
    }

    macro_rules! test_ld_reg_at_hl {
        ($name:ident, $opcode:expr, $register:ident) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[]);
                cpu.registers.set_hl(MEM_ADDR);
                bus.write_u8(MEM_ADDR, 0x12);
                cpu.registers.$register = 0x00;

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 8);
                assert_eq!(cpu.registers.$register, 0x12);
                assert_eq!(cpu.registers.get_hl(), MEM_ADDR);
            }
        };
    }

    macro_rules! test_ld_at_hl_reg {
        ($name:ident, $opcode:expr, $register:ident) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[]);
                cpu.registers.set_hl(MEM_ADDR);
                cpu.registers.$register = 0x12;

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 8);
                assert_eq!(bus.read_u8(MEM_ADDR), 0x12);
                assert_eq!(cpu.registers.get_hl(), MEM_ADDR);
            }
        };
    }

    test_ld_at_pair_a!(execute_0x02, 0x02, set_bc);
    test_ld_a_at_pair!(execute_0x0A, 0x0A, set_bc);
    test_ld_at_pair_a!(execute_0x12, 0x12, set_de);
    test_ld_a_at_pair!(execute_0x1A, 0x1A, set_de);

    #[test]
    fn execute_0x22() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_hl(MEM_ADDR);
        cpu.registers.a = 0x12;

        let cycles = cpu.execute(0x22, &mut bus);

        assert_eq!(cycles, 8);
        assert_eq!(bus.read_u8(MEM_ADDR), 0x12);
        assert_eq!(cpu.registers.get_hl(), MEM_ADDR + 1);
    }

    #[test]
    fn execute_0x2A() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_hl(MEM_ADDR);
        bus.write_u8(MEM_ADDR, 0x12);
        cpu.registers.a = 0x00;

        let cycles = cpu.execute(0x2A, &mut bus);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.a, 0x12);
        assert_eq!(cpu.registers.get_hl(), MEM_ADDR + 1);
    }

    #[test]
    fn execute_0x32() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_hl(MEM_ADDR);
        cpu.registers.a = 0x12;

        let cycles = cpu.execute(0x32, &mut bus);

        assert_eq!(cycles, 8);
        assert_eq!(bus.read_u8(MEM_ADDR), 0x12);
        assert_eq!(cpu.registers.get_hl(), MEM_ADDR - 1);
    }

    #[test]
    fn execute_0x36() {
        let (mut cpu, mut bus) = setup(&[0x12]);
        cpu.registers.set_hl(MEM_ADDR);

        let cycles = cpu.execute(0x36, &mut bus);

        assert_eq!(cycles, 12);
        assert_eq!(bus.read_u8(MEM_ADDR), 0x12);
        assert_eq!(cpu.registers.get_hl(), MEM_ADDR);
    }

    #[test]
    fn execute_0x3A() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_hl(MEM_ADDR);
        bus.write_u8(MEM_ADDR, 0x12);
        cpu.registers.a = 0x00;

        let cycles = cpu.execute(0x3A, &mut bus);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.a, 0x12);
        assert_eq!(cpu.registers.get_hl(), MEM_ADDR - 1);
    }

    test_ld_reg_at_hl!(execute_0x46, 0x46, b);
    test_ld_reg_at_hl!(execute_0x4E, 0x4E, c);
    test_ld_reg_at_hl!(execute_0x56, 0x56, d);
    test_ld_reg_at_hl!(execute_0x5E, 0x5E, e);
    test_ld_reg_at_hl!(execute_0x7E, 0x7E, a);

    #[test]
    fn execute_0x66() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_hl(MEM_ADDR);
        bus.write_u8(MEM_ADDR, 0x12);

        let cycles = cpu.execute(0x66, &mut bus);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.h, 0x12);
        assert_eq!(cpu.registers.get_hl(), 0x1223);
    }

    #[test]
    fn execute_0x6E() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_hl(MEM_ADDR);
        bus.write_u8(MEM_ADDR, 0x12);

        let cycles = cpu.execute(0x6E, &mut bus);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.l, 0x12);
        assert_eq!(cpu.registers.get_hl(), 0xC112);
    }

    test_ld_at_hl_reg!(execute_0x70, 0x70, b);
    test_ld_at_hl_reg!(execute_0x71, 0x71, c);
    test_ld_at_hl_reg!(execute_0x72, 0x72, d);
    test_ld_at_hl_reg!(execute_0x73, 0x73, e);
    test_ld_at_hl_reg!(execute_0x77, 0x77, a);

    #[test]
    fn execute_0x74() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_hl(MEM_ADDR);

        let cycles = cpu.execute(0x74, &mut bus);

        assert_eq!(cycles, 8);
        assert_eq!(bus.read_u8(MEM_ADDR), 0xC1);
        assert_eq!(cpu.registers.get_hl(), MEM_ADDR);
    }

    #[test]
    fn execute_0x75() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_hl(MEM_ADDR);

        let cycles = cpu.execute(0x75, &mut bus);

        assert_eq!(cycles, 8);
        assert_eq!(bus.read_u8(MEM_ADDR), 0x23);
        assert_eq!(cpu.registers.get_hl(), MEM_ADDR);
    }

    #[test]
    fn execute_0xE0() {
        let (mut cpu, mut bus) = setup(&[0x42]);
        cpu.registers.a = 0x12;

        let cycles = cpu.execute(0xE0, &mut bus);

        assert_eq!(cycles, 12);
        assert_eq!(bus.read_u8(IO_ADDR), 0x12);
        assert_eq!(cpu.registers.a, 0x12);
    }

    #[test]
    fn execute_0xE2() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.c = 0x42;
        cpu.registers.a = 0x12;

        let cycles = cpu.execute(0xE2, &mut bus);

        assert_eq!(cycles, 8);
        assert_eq!(bus.read_u8(IO_ADDR), 0x12);
        assert_eq!(cpu.registers.a, 0x12);
    }

    #[test]
    fn execute_0xEA() {
        let (mut cpu, mut bus) = setup(&[0x23, 0xC1]);
        cpu.registers.a = 0x12;

        let cycles = cpu.execute(0xEA, &mut bus);

        assert_eq!(cycles, 16);
        assert_eq!(bus.read_u8(MEM_ADDR), 0x12);
        assert_eq!(cpu.registers.a, 0x12);
    }

    #[test]
    fn execute_0xF0() {
        let (mut cpu, mut bus) = setup(&[0x42]);
        bus.write_u8(IO_ADDR, 0x12);
        cpu.registers.a = 0x00;

        let cycles = cpu.execute(0xF0, &mut bus);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.a, 0x12);
    }

    #[test]
    fn execute_0xF2() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.c = 0x42;
        bus.write_u8(IO_ADDR, 0x12);
        cpu.registers.a = 0x00;

        let cycles = cpu.execute(0xF2, &mut bus);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.a, 0x12);
    }

    #[test]
    fn execute_0xFA() {
        let (mut cpu, mut bus) = setup(&[0x23, 0xC1]);
        bus.write_u8(MEM_ADDR, 0x12);
        cpu.registers.a = 0x00;

        let cycles = cpu.execute(0xFA, &mut bus);

        assert_eq!(cycles, 16);
        assert_eq!(cpu.registers.a, 0x12);
    }
}
