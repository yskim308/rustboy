#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use crate::{
        bus::{Bus, cartridge::Cartridge},
        cpu::Cpu,
    };

    const MEM_ADDR: u16 = 0xC123;

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

    fn assert_flags(cpu: &Cpu, z: bool, n: bool, h: bool, c: bool) {
        assert_eq!(cpu.registers.get_z(), z);
        assert_eq!(cpu.registers.get_n(), n);
        assert_eq!(cpu.registers.get_h(), h);
        assert_eq!(cpu.registers.get_c(), c);
    }

    macro_rules! test_alu_r {
        ($name:ident, $opcode:expr, $register:ident, $setup:expr, $assertion:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[]);
                $setup(&mut cpu);
                cpu.registers.$register = 0x01;

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 4);
                $assertion(&cpu, &bus);
            }
        };
    }

    macro_rules! test_alu_at_hl {
        ($name:ident, $opcode:expr, $setup:expr, $assertion:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[]);
                cpu.registers.set_hl(MEM_ADDR);
                bus.write_u8(MEM_ADDR, 0x01);
                $setup(&mut cpu);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 8);
                $assertion(&cpu, &bus);
            }
        };
    }

    macro_rules! test_alu_u8 {
        ($name:ident, $opcode:expr, $setup:expr, $assertion:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[0x01]);
                $setup(&mut cpu);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 8);
                $assertion(&cpu, &bus);
            }
        };
    }

    macro_rules! test_inc_r {
        ($name:ident, $opcode:expr, $register:ident) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[]);
                cpu.registers.$register = 0x0F;
                cpu.registers.set_c(true);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 4);
                assert_eq!(cpu.registers.$register, 0x10);
                assert_flags(&cpu, false, false, true, true);
            }
        };
    }

    macro_rules! test_dec_r {
        ($name:ident, $opcode:expr, $register:ident) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[]);
                cpu.registers.$register = 0x10;
                cpu.registers.set_c(true);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 4);
                assert_eq!(cpu.registers.$register, 0x0F);
                assert_flags(&cpu, false, true, true, true);
            }
        };
    }

    test_inc_r!(execute_0x04, 0x04, b);
    test_dec_r!(execute_0x05, 0x05, b);
    test_inc_r!(execute_0x0C, 0x0C, c);
    test_dec_r!(execute_0x0D, 0x0D, c);
    test_inc_r!(execute_0x14, 0x14, d);
    test_dec_r!(execute_0x15, 0x15, d);
    test_inc_r!(execute_0x1C, 0x1C, e);
    test_dec_r!(execute_0x1D, 0x1D, e);
    test_inc_r!(execute_0x24, 0x24, h);
    test_dec_r!(execute_0x25, 0x25, h);
    test_inc_r!(execute_0x2C, 0x2C, l);
    test_dec_r!(execute_0x2D, 0x2D, l);
    test_inc_r!(execute_0x3C, 0x3C, a);
    test_dec_r!(execute_0x3D, 0x3D, a);

    #[test]
    fn execute_0x34() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_hl(MEM_ADDR);
        bus.write_u8(MEM_ADDR, 0x0F);
        cpu.registers.set_c(true);

        let cycles = cpu.execute(0x34, &mut bus);

        assert_eq!(cycles, 12);
        assert_eq!(bus.read_u8(MEM_ADDR), 0x10);
        assert_flags(&cpu, false, false, true, true);
    }

    #[test]
    fn execute_0x35() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_hl(MEM_ADDR);
        bus.write_u8(MEM_ADDR, 0x10);
        cpu.registers.set_c(true);

        let cycles = cpu.execute(0x35, &mut bus);

        assert_eq!(cycles, 12);
        assert_eq!(bus.read_u8(MEM_ADDR), 0x0F);
        assert_flags(&cpu, false, true, true, true);
    }

    test_alu_r!(execute_0x80, 0x80, b, |cpu: &mut Cpu| cpu.registers.a = 0x0F, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, false, true, false);
    });
    test_alu_r!(execute_0x81, 0x81, c, |cpu: &mut Cpu| cpu.registers.a = 0x0F, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, false, true, false);
    });
    test_alu_r!(execute_0x82, 0x82, d, |cpu: &mut Cpu| cpu.registers.a = 0x0F, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, false, true, false);
    });
    test_alu_r!(execute_0x83, 0x83, e, |cpu: &mut Cpu| cpu.registers.a = 0x0F, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, false, true, false);
    });
    test_alu_r!(execute_0x84, 0x84, h, |cpu: &mut Cpu| cpu.registers.a = 0x0F, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, false, true, false);
    });
    test_alu_r!(execute_0x85, 0x85, l, |cpu: &mut Cpu| cpu.registers.a = 0x0F, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, false, true, false);
    });
    test_alu_at_hl!(execute_0x86, 0x86, |cpu: &mut Cpu| cpu.registers.a = 0x0F, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, false, true, false);
    });
    #[test]
    fn execute_0x87() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.a = 0x08;

        let cycles = cpu.execute(0x87, &mut bus);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(&cpu, false, false, true, false);
    }
    test_alu_u8!(execute_0xC6, 0xC6, |cpu: &mut Cpu| cpu.registers.a = 0x0F, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, false, true, false);
    });

    test_alu_r!(execute_0x88, 0x88, b, |cpu: &mut Cpu| { cpu.registers.a = 0x0F; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x11);
        assert_flags(cpu, false, false, true, false);
    });
    test_alu_r!(execute_0x89, 0x89, c, |cpu: &mut Cpu| { cpu.registers.a = 0x0F; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x11);
        assert_flags(cpu, false, false, true, false);
    });
    test_alu_r!(execute_0x8A, 0x8A, d, |cpu: &mut Cpu| { cpu.registers.a = 0x0F; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x11);
        assert_flags(cpu, false, false, true, false);
    });
    test_alu_r!(execute_0x8B, 0x8B, e, |cpu: &mut Cpu| { cpu.registers.a = 0x0F; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x11);
        assert_flags(cpu, false, false, true, false);
    });
    test_alu_r!(execute_0x8C, 0x8C, h, |cpu: &mut Cpu| { cpu.registers.a = 0x0F; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x11);
        assert_flags(cpu, false, false, true, false);
    });
    test_alu_r!(execute_0x8D, 0x8D, l, |cpu: &mut Cpu| { cpu.registers.a = 0x0F; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x11);
        assert_flags(cpu, false, false, true, false);
    });
    test_alu_at_hl!(execute_0x8E, 0x8E, |cpu: &mut Cpu| { cpu.registers.a = 0x0F; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x11);
        assert_flags(cpu, false, false, true, false);
    });
    #[test]
    fn execute_0x8F() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.a = 0x08;
        cpu.registers.set_c(true);

        let cycles = cpu.execute(0x8F, &mut bus);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0x11);
        assert_flags(&cpu, false, false, true, false);
    }
    test_alu_u8!(execute_0xCE, 0xCE, |cpu: &mut Cpu| { cpu.registers.a = 0x0F; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x11);
        assert_flags(cpu, false, false, true, false);
    });

    test_alu_r!(execute_0x90, 0x90, b, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0F);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0x91, 0x91, c, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0F);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0x92, 0x92, d, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0F);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0x93, 0x93, e, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0F);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0x94, 0x94, h, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0F);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0x95, 0x95, l, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0F);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_at_hl!(execute_0x96, 0x96, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0F);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0x97, 0x97, a, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags(cpu, true, true, false, false);
    });
    test_alu_u8!(execute_0xD6, 0xD6, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0F);
        assert_flags(cpu, false, true, true, false);
    });

    test_alu_r!(execute_0x98, 0x98, b, |cpu: &mut Cpu| { cpu.registers.a = 0x10; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0E);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0x99, 0x99, c, |cpu: &mut Cpu| { cpu.registers.a = 0x10; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0E);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0x9A, 0x9A, d, |cpu: &mut Cpu| { cpu.registers.a = 0x10; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0E);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0x9B, 0x9B, e, |cpu: &mut Cpu| { cpu.registers.a = 0x10; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0E);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0x9C, 0x9C, h, |cpu: &mut Cpu| { cpu.registers.a = 0x10; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0E);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0x9D, 0x9D, l, |cpu: &mut Cpu| { cpu.registers.a = 0x10; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0E);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_at_hl!(execute_0x9E, 0x9E, |cpu: &mut Cpu| { cpu.registers.a = 0x10; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0E);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0x9F, 0x9F, a, |cpu: &mut Cpu| { cpu.registers.a = 0x10; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xFF);
        assert_flags(cpu, false, true, true, true);
    });
    test_alu_u8!(execute_0xDE, 0xDE, |cpu: &mut Cpu| { cpu.registers.a = 0x10; cpu.registers.set_c(true); }, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x0E);
        assert_flags(cpu, false, true, true, false);
    });

    test_alu_r!(execute_0xA0, 0xA0, b, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags(cpu, true, false, true, false);
    });
    test_alu_r!(execute_0xA1, 0xA1, c, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags(cpu, true, false, true, false);
    });
    test_alu_r!(execute_0xA2, 0xA2, d, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags(cpu, true, false, true, false);
    });
    test_alu_r!(execute_0xA3, 0xA3, e, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags(cpu, true, false, true, false);
    });
    test_alu_r!(execute_0xA4, 0xA4, h, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags(cpu, true, false, true, false);
    });
    test_alu_r!(execute_0xA5, 0xA5, l, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags(cpu, true, false, true, false);
    });
    test_alu_at_hl!(execute_0xA6, 0xA6, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags(cpu, true, false, true, false);
    });
    #[test]
    fn execute_0xA7() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.a = 0xF0;

        let cycles = cpu.execute(0xA7, &mut bus);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0xF0);
        assert_flags(&cpu, false, false, true, false);
    }
    test_alu_u8!(execute_0xE6, 0xE6, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags(cpu, true, false, true, false);
    });

    test_alu_r!(execute_0xA8, 0xA8, b, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });
    test_alu_r!(execute_0xA9, 0xA9, c, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });
    test_alu_r!(execute_0xAA, 0xAA, d, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });
    test_alu_r!(execute_0xAB, 0xAB, e, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });
    test_alu_r!(execute_0xAC, 0xAC, h, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });
    test_alu_r!(execute_0xAD, 0xAD, l, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });
    test_alu_at_hl!(execute_0xAE, 0xAE, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });
    #[test]
    fn execute_0xAF() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.a = 0xF0;

        let cycles = cpu.execute(0xAF, &mut bus);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags(&cpu, true, false, false, false);
    }
    test_alu_u8!(execute_0xEE, 0xEE, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });

    test_alu_r!(execute_0xB0, 0xB0, b, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });
    test_alu_r!(execute_0xB1, 0xB1, c, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });
    test_alu_r!(execute_0xB2, 0xB2, d, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });
    test_alu_r!(execute_0xB3, 0xB3, e, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });
    test_alu_r!(execute_0xB4, 0xB4, h, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });
    test_alu_r!(execute_0xB5, 0xB5, l, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });
    test_alu_at_hl!(execute_0xB6, 0xB6, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });
    #[test]
    fn execute_0xB7() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.a = 0xF0;

        let cycles = cpu.execute(0xB7, &mut bus);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0xF0);
        assert_flags(&cpu, false, false, false, false);
    }
    test_alu_u8!(execute_0xF6, 0xF6, |cpu: &mut Cpu| cpu.registers.a = 0xF0, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0xF1);
        assert_flags(cpu, false, false, false, false);
    });

    test_alu_r!(execute_0xB8, 0xB8, b, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0xB9, 0xB9, c, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0xBA, 0xBA, d, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0xBB, 0xBB, e, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0xBC, 0xBC, h, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_r!(execute_0xBD, 0xBD, l, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, true, true, false);
    });
    test_alu_at_hl!(execute_0xBE, 0xBE, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, true, true, false);
    });
    #[test]
    fn execute_0xBF() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.a = 0x10;

        let cycles = cpu.execute(0xBF, &mut bus);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(&cpu, true, true, false, false);
    }
    test_alu_u8!(execute_0xFE, 0xFE, |cpu: &mut Cpu| cpu.registers.a = 0x10, |cpu: &Cpu, _| {
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(cpu, false, true, true, false);
    });

    #[test]
    fn execute_0x27() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.a = 0x0A;
        cpu.registers.set_n(false);
        cpu.registers.set_h(false);
        cpu.registers.set_c(false);

        let cycles = cpu.execute(0x27, &mut bus);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags(&cpu, false, false, false, false);
    }

    #[test]
    fn execute_0x2F() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.a = 0x0F;
        cpu.registers.set_z(true);
        cpu.registers.set_n(false);
        cpu.registers.set_h(false);
        cpu.registers.set_c(true);

        let cycles = cpu.execute(0x2F, &mut bus);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0xF0);
        assert_flags(&cpu, true, true, true, true);
    }

    #[test]
    fn execute_0x37() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_z(true);
        cpu.registers.set_n(true);
        cpu.registers.set_h(true);
        cpu.registers.set_c(false);

        let cycles = cpu.execute(0x37, &mut bus);

        assert_eq!(cycles, 4);
        assert_flags(&cpu, true, false, false, true);
    }

    #[test]
    fn execute_0x3F() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_z(true);
        cpu.registers.set_n(true);
        cpu.registers.set_h(true);
        cpu.registers.set_c(true);

        let cycles = cpu.execute(0x3F, &mut bus);

        assert_eq!(cycles, 4);
        assert_flags(&cpu, true, false, false, false);
    }
}
