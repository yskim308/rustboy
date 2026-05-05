#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

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

    fn assert_flags(cpu: &Cpu, z: bool, n: bool, h: bool, c: bool) {
        assert_eq!(cpu.registers.get_z(), z);
        assert_eq!(cpu.registers.get_n(), n);
        assert_eq!(cpu.registers.get_h(), h);
        assert_eq!(cpu.registers.get_c(), c);
    }

    macro_rules! test_inc_rr {
        ($name:ident, $opcode:expr, $setter:ident, $getter:ident) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[]);
                cpu.registers.$setter(0x1234);
                cpu.registers.set_z(true);
                cpu.registers.set_n(true);
                cpu.registers.set_h(true);
                cpu.registers.set_c(true);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 8);
                assert_eq!(cpu.registers.$getter(), 0x1235);
                assert_flags(&cpu, true, true, true, true);
            }
        };
    }

    macro_rules! test_dec_rr {
        ($name:ident, $opcode:expr, $setter:ident, $getter:ident) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[]);
                cpu.registers.$setter(0x1234);
                cpu.registers.set_z(true);
                cpu.registers.set_n(true);
                cpu.registers.set_h(true);
                cpu.registers.set_c(true);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 8);
                assert_eq!(cpu.registers.$getter(), 0x1233);
                assert_flags(&cpu, true, true, true, true);
            }
        };
    }

    macro_rules! test_add_hl_rr {
        ($name:ident, $opcode:expr, $setup_rr:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[]);
                cpu.registers.set_hl(0x0FFF);
                cpu.registers.set_z(true);
                cpu.registers.set_n(true);
                cpu.registers.set_h(false);
                cpu.registers.set_c(false);
                $setup_rr(&mut cpu);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 8);
                assert_eq!(cpu.registers.get_hl(), 0x1000);
                assert_flags(&cpu, true, false, true, false);
            }
        };
    }

    test_inc_rr!(execute_0x03, 0x03, set_bc, get_bc);
    test_dec_rr!(execute_0x0B, 0x0B, set_bc, get_bc);
    test_inc_rr!(execute_0x13, 0x13, set_de, get_de);
    test_dec_rr!(execute_0x1B, 0x1B, set_de, get_de);
    test_inc_rr!(execute_0x23, 0x23, set_hl, get_hl);
    test_dec_rr!(execute_0x2B, 0x2B, set_hl, get_hl);

    test_add_hl_rr!(execute_0x09, 0x09, |cpu: &mut Cpu| cpu.registers.set_bc(0x0001));
    test_add_hl_rr!(execute_0x19, 0x19, |cpu: &mut Cpu| cpu.registers.set_de(0x0001));
    #[test]
    fn execute_0x39() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_hl(0x0FFF);
        cpu.registers.sp = 0x0001;
        cpu.registers.set_z(true);
        cpu.registers.set_n(true);
        cpu.registers.set_h(false);
        cpu.registers.set_c(false);

        let cycles = cpu.execute(0x39, &mut bus);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.get_hl(), 0x1000);
        assert_flags(&cpu, true, false, true, false);
    }

    #[test]
    fn execute_0x29() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_hl(0x0800);
        cpu.registers.set_z(true);
        cpu.registers.set_n(true);
        cpu.registers.set_h(false);
        cpu.registers.set_c(false);

        let cycles = cpu.execute(0x29, &mut bus);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.get_hl(), 0x1000);
        assert_flags(&cpu, true, false, true, false);
    }

    #[test]
    fn execute_0xE8() {
        let (mut cpu, mut bus) = setup(&[0x01]);
        cpu.registers.sp = 0x00FF;
        cpu.registers.set_z(true);
        cpu.registers.set_n(true);
        cpu.registers.set_h(false);
        cpu.registers.set_c(false);

        let cycles = cpu.execute(0xE8, &mut bus);

        assert_eq!(cycles, 16);
        assert_eq!(cpu.registers.sp, 0x0100);
        assert_flags(&cpu, false, false, true, true);
    }

    #[test]
    fn execute_0xF8() {
        let (mut cpu, mut bus) = setup(&[0x01]);
        cpu.registers.sp = 0x00FF;
        cpu.registers.set_z(true);
        cpu.registers.set_n(true);
        cpu.registers.set_h(false);
        cpu.registers.set_c(false);

        let cycles = cpu.execute(0xF8, &mut bus);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.get_hl(), 0x0100);
        assert_eq!(cpu.registers.sp, 0x00FF);
        assert_flags(&cpu, false, false, true, true);
    }
}
