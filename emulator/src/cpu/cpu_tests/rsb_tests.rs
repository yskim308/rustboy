#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use crate::{
        bus::{Bus, cartridge::Cartridge},
        cpu::Cpu,
    };

    const MEM_ADDR: u16 = 0xC123;

    fn setup(cb_opcode: u8) -> (Cpu, Bus) {
        let mut cpu = Cpu::new();
        cpu.registers.pc = 0x0100;

        let mut rom = vec![0; 0x8000];
        rom[0x0100] = cb_opcode;

        let bus = Bus::new(Cartridge::new(rom));
        (cpu, bus)
    }

    fn assert_flags(cpu: &Cpu, z: bool, n: bool, h: bool, c: bool) {
        assert_eq!(cpu.registers.get_z(), z);
        assert_eq!(cpu.registers.get_n(), n);
        assert_eq!(cpu.registers.get_h(), h);
        assert_eq!(cpu.registers.get_c(), c);
    }

    macro_rules! test_cb_reg {
        ($name:ident, $cb_opcode:expr, $setup:expr, $assertion:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup($cb_opcode);
                $setup(&mut cpu);

                let cycles = cpu.execute(0xCB, &mut bus);

                assert_eq!(cycles, 8);
                $assertion(&cpu, &bus);
            }
        };
    }

    macro_rules! test_cb_hl {
        ($name:ident, $cb_opcode:expr, $cycles:expr, $value:expr, $setup:expr, $assertion:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup($cb_opcode);
                cpu.registers.set_hl(MEM_ADDR);
                bus.write_u8(MEM_ADDR, $value);
                $setup(&mut cpu);

                let cycles = cpu.execute(0xCB, &mut bus);

                assert_eq!(cycles, $cycles);
                $assertion(&cpu, &bus);
            }
        };
    }

    #[test]
    fn helper_rlc_u8() {
        let cpu = Cpu::new();
        assert_eq!(cpu.rlc_u8(0x80), (0x01, true));
    }

    #[test]
    fn helper_rrc_u8() {
        let cpu = Cpu::new();
        assert_eq!(cpu.rrc_u8(0x01), (0x80, true));
    }

    #[test]
    fn helper_rl_u8() {
        let mut cpu = Cpu::new();
        cpu.registers.set_c(true);
        assert_eq!(cpu.rl_u8(0x80), (0x01, true));
    }

    #[test]
    fn helper_rr_u8() {
        let mut cpu = Cpu::new();
        cpu.registers.set_c(true);
        assert_eq!(cpu.rr_u8(0x01), (0x80, true));
    }

    #[test]
    fn helper_sla_u8() {
        let cpu = Cpu::new();
        assert_eq!(cpu.sla_u8(0x80), (0x00, true));
    }

    #[test]
    fn helper_sra_u8() {
        let cpu = Cpu::new();
        assert_eq!(cpu.sra_u8(0x81), (0xC0, true));
    }

    #[test]
    fn helper_swap_u8() {
        let cpu = Cpu::new();
        assert_eq!(cpu.swap_u8(0xF0), 0x0F);
    }

    #[test]
    fn helper_srl_u8() {
        let cpu = Cpu::new();
        assert_eq!(cpu.srl_u8(0x01), (0x00, true));
    }

    #[test]
    fn helper_bit_u8() {
        let mut cpu = Cpu::new();
        cpu.registers.set_c(true);

        cpu.bit_u8(3, 0xF7);

        assert_flags(&cpu, true, false, true, true);
    }

    #[test]
    fn helper_res_u8() {
        let cpu = Cpu::new();
        assert_eq!(cpu.res_u8(0, 0xFF), 0xFE);
    }

    #[test]
    fn helper_set_u8() {
        let cpu = Cpu::new();
        assert_eq!(cpu.set_u8(0, 0x00), 0x01);
    }

    test_cb_reg!(
        execute_cb_0x00,
        0x00,
        |cpu: &mut Cpu| {
            cpu.registers.b = 0x80;
        },
        |cpu: &Cpu, _| {
            assert_eq!(cpu.registers.b, 0x01);
            assert_flags(cpu, false, false, false, true);
        }
    );

    test_cb_hl!(
        execute_cb_0x06,
        0x06,
        16,
        0x80,
        |_cpu: &mut Cpu| {},
        |cpu: &Cpu, bus: &Bus| {
            assert_eq!(bus.read_u8(MEM_ADDR), 0x01);
            assert_flags(cpu, false, false, false, true);
        }
    );

    test_cb_reg!(
        execute_cb_0x08,
        0x08,
        |cpu: &mut Cpu| {
            cpu.registers.b = 0x01;
        },
        |cpu: &Cpu, _| {
            assert_eq!(cpu.registers.b, 0x80);
            assert_flags(cpu, false, false, false, true);
        }
    );

    test_cb_hl!(
        execute_cb_0x0E,
        0x0E,
        16,
        0x01,
        |_cpu: &mut Cpu| {},
        |cpu: &Cpu, bus: &Bus| {
            assert_eq!(bus.read_u8(MEM_ADDR), 0x80);
            assert_flags(cpu, false, false, false, true);
        }
    );

    test_cb_reg!(
        execute_cb_0x10,
        0x10,
        |cpu: &mut Cpu| {
            cpu.registers.b = 0x80;
            cpu.registers.set_c(true);
        },
        |cpu: &Cpu, _| {
            assert_eq!(cpu.registers.b, 0x01);
            assert_flags(cpu, false, false, false, true);
        }
    );

    test_cb_hl!(
        execute_cb_0x16,
        0x16,
        16,
        0x80,
        |cpu: &mut Cpu| {
            cpu.registers.set_c(true);
        },
        |cpu: &Cpu, bus: &Bus| {
            assert_eq!(bus.read_u8(MEM_ADDR), 0x01);
            assert_flags(cpu, false, false, false, true);
        }
    );

    test_cb_reg!(
        execute_cb_0x18,
        0x18,
        |cpu: &mut Cpu| {
            cpu.registers.b = 0x01;
            cpu.registers.set_c(true);
        },
        |cpu: &Cpu, _| {
            assert_eq!(cpu.registers.b, 0x80);
            assert_flags(cpu, false, false, false, true);
        }
    );

    test_cb_hl!(
        execute_cb_0x1E,
        0x1E,
        16,
        0x01,
        |cpu: &mut Cpu| {
            cpu.registers.set_c(true);
        },
        |cpu: &Cpu, bus: &Bus| {
            assert_eq!(bus.read_u8(MEM_ADDR), 0x80);
            assert_flags(cpu, false, false, false, true);
        }
    );

    test_cb_reg!(
        execute_cb_0x20,
        0x20,
        |cpu: &mut Cpu| {
            cpu.registers.b = 0x80;
        },
        |cpu: &Cpu, _| {
            assert_eq!(cpu.registers.b, 0x00);
            assert_flags(cpu, true, false, false, true);
        }
    );

    test_cb_hl!(
        execute_cb_0x26,
        0x26,
        16,
        0x80,
        |_cpu: &mut Cpu| {},
        |cpu: &Cpu, bus: &Bus| {
            assert_eq!(bus.read_u8(MEM_ADDR), 0x00);
            assert_flags(cpu, true, false, false, true);
        }
    );

    test_cb_reg!(
        execute_cb_0x28,
        0x28,
        |cpu: &mut Cpu| {
            cpu.registers.b = 0x81;
        },
        |cpu: &Cpu, _| {
            assert_eq!(cpu.registers.b, 0xC0);
            assert_flags(cpu, false, false, false, true);
        }
    );

    test_cb_hl!(
        execute_cb_0x2E,
        0x2E,
        16,
        0x81,
        |_cpu: &mut Cpu| {},
        |cpu: &Cpu, bus: &Bus| {
            assert_eq!(bus.read_u8(MEM_ADDR), 0xC0);
            assert_flags(cpu, false, false, false, true);
        }
    );

    test_cb_reg!(
        execute_cb_0x30,
        0x30,
        |cpu: &mut Cpu| {
            cpu.registers.b = 0xF0;
        },
        |cpu: &Cpu, _| {
            assert_eq!(cpu.registers.b, 0x0F);
            assert_flags(cpu, false, false, false, false);
        }
    );

    test_cb_hl!(
        execute_cb_0x36,
        0x36,
        16,
        0xF0,
        |_cpu: &mut Cpu| {},
        |cpu: &Cpu, bus: &Bus| {
            assert_eq!(bus.read_u8(MEM_ADDR), 0x0F);
            assert_flags(cpu, false, false, false, false);
        }
    );

    test_cb_reg!(
        execute_cb_0x38,
        0x38,
        |cpu: &mut Cpu| {
            cpu.registers.b = 0x01;
        },
        |cpu: &Cpu, _| {
            assert_eq!(cpu.registers.b, 0x00);
            assert_flags(cpu, true, false, false, true);
        }
    );

    test_cb_hl!(
        execute_cb_0x3E,
        0x3E,
        16,
        0x01,
        |_cpu: &mut Cpu| {},
        |cpu: &Cpu, bus: &Bus| {
            assert_eq!(bus.read_u8(MEM_ADDR), 0x00);
            assert_flags(cpu, true, false, false, true);
        }
    );

    test_cb_reg!(
        execute_cb_0x40,
        0x40,
        |cpu: &mut Cpu| {
            cpu.registers.b = 0x00;
            cpu.registers.set_c(true);
        },
        |cpu: &Cpu, _| {
            assert_eq!(cpu.registers.b, 0x00);
            assert_flags(cpu, true, false, true, true);
        }
    );

    test_cb_hl!(
        execute_cb_0x46,
        0x46,
        12,
        0x01,
        |cpu: &mut Cpu| {
            cpu.registers.set_c(true);
        },
        |cpu: &Cpu, bus: &Bus| {
            assert_eq!(bus.read_u8(MEM_ADDR), 0x01);
            assert_flags(cpu, false, false, true, true);
        }
    );

    test_cb_reg!(
        execute_cb_0x80,
        0x80,
        |cpu: &mut Cpu| {
            cpu.registers.b = 0xFF;
            cpu.registers.set_z(true);
            cpu.registers.set_n(true);
            cpu.registers.set_h(true);
            cpu.registers.set_c(true);
        },
        |cpu: &Cpu, _| {
            assert_eq!(cpu.registers.b, 0xFE);
            assert_flags(cpu, true, true, true, true);
        }
    );

    test_cb_hl!(
        execute_cb_0x86,
        0x86,
        16,
        0xFF,
        |cpu: &mut Cpu| {
            cpu.registers.set_z(true);
            cpu.registers.set_n(true);
            cpu.registers.set_h(true);
            cpu.registers.set_c(true);
        },
        |cpu: &Cpu, bus: &Bus| {
            assert_eq!(bus.read_u8(MEM_ADDR), 0xFE);
            assert_flags(cpu, true, true, true, true);
        }
    );

    test_cb_reg!(
        execute_cb_0xC0,
        0xC0,
        |cpu: &mut Cpu| {
            cpu.registers.b = 0x00;
            cpu.registers.set_z(true);
            cpu.registers.set_n(true);
            cpu.registers.set_h(true);
            cpu.registers.set_c(true);
        },
        |cpu: &Cpu, _| {
            assert_eq!(cpu.registers.b, 0x01);
            assert_flags(cpu, true, true, true, true);
        }
    );

    test_cb_hl!(
        execute_cb_0xC6,
        0xC6,
        16,
        0x00,
        |cpu: &mut Cpu| {
            cpu.registers.set_z(true);
            cpu.registers.set_n(true);
            cpu.registers.set_h(true);
            cpu.registers.set_c(true);
        },
        |cpu: &Cpu, bus: &Bus| {
            assert_eq!(bus.read_u8(MEM_ADDR), 0x01);
            assert_flags(cpu, true, true, true, true);
        }
    );
}
