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
        cpu.registers.sp = 0xFFFE;

        let mut rom = vec![0; 0x8000];
        for (i, &byte) in instruction_bytes.iter().enumerate() {
            rom[0x0100 + i] = byte;
        }

        let bus = Bus::new(Cartridge::new(rom));
        (cpu, bus)
    }

    fn write_return_address(bus: &mut Bus, sp: u16, address: u16) {
        let [low, high] = address.to_le_bytes();
        bus.write_u8(sp, low);
        bus.write_u8(sp.wrapping_add(1), high);
    }

    fn assert_stack_address(bus: &Bus, sp: u16, address: u16) {
        let [low, high] = address.to_le_bytes();
        assert_eq!(bus.read_u8(sp), low);
        assert_eq!(bus.read_u8(sp.wrapping_add(1)), high);
    }

    macro_rules! test_jr {
        ($name:ident, $opcode:expr, $offset:expr, $setup:expr, $cycles:expr, $pc:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[$offset as u8]);
                $setup(&mut cpu);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, $cycles);
                assert_eq!(cpu.registers.pc, $pc);
            }
        };
    }

    macro_rules! test_jp {
        ($name:ident, $opcode:expr, $setup:expr, $cycles:expr, $pc:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[0x34, 0x12]);
                $setup(&mut cpu);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, $cycles);
                assert_eq!(cpu.registers.pc, $pc);
            }
        };
    }

    macro_rules! test_call {
        ($name:ident, $opcode:expr, $setup:expr, $cycles:expr, $pc:expr, $sp:expr, $check_stack:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[0x34, 0x12]);
                cpu.registers.sp = 0xFFFE;
                $setup(&mut cpu);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, $cycles);
                assert_eq!(cpu.registers.pc, $pc);
                assert_eq!(cpu.registers.sp, $sp);
                if $check_stack {
                    assert_stack_address(&bus, $sp, 0x0102);
                }
            }
        };
    }

    macro_rules! test_ret {
        ($name:ident, $opcode:expr, $setup:expr, $cycles:expr, $pc:expr, $sp:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[]);
                cpu.registers.sp = 0xFFFC;
                write_return_address(&mut bus, 0xFFFC, 0x1234);
                $setup(&mut cpu);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, $cycles);
                assert_eq!(cpu.registers.pc, $pc);
                assert_eq!(cpu.registers.sp, $sp);
            }
        };
    }

    macro_rules! test_rst {
        ($name:ident, $opcode:expr, $target:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[]);
                cpu.registers.sp = 0xFFFE;

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 16);
                assert_eq!(cpu.registers.pc, $target);
                assert_eq!(cpu.registers.sp, 0xFFFC);
                assert_stack_address(&bus, 0xFFFC, 0x0100);
            }
        };
    }

    test_jr!(execute_0x18, 0x18, 0x05, |_| {}, 12, 0x0106);
    test_jr!(
        execute_0x20_taken,
        0x20,
        0x05,
        |cpu: &mut Cpu| cpu.registers.set_z(false),
        12,
        0x0106
    );
    test_jr!(
        execute_0x20_not_taken,
        0x20,
        0x05,
        |cpu: &mut Cpu| cpu.registers.set_z(true),
        8,
        0x0101
    );
    test_jr!(
        execute_0x28_taken,
        0x28,
        0x05,
        |cpu: &mut Cpu| cpu.registers.set_z(true),
        12,
        0x0106
    );
    test_jr!(
        execute_0x28_not_taken,
        0x28,
        0x05,
        |cpu: &mut Cpu| cpu.registers.set_z(false),
        8,
        0x0101
    );
    test_jr!(
        execute_0x30_taken,
        0x30,
        0x05,
        |cpu: &mut Cpu| cpu.registers.set_c(false),
        12,
        0x0106
    );
    test_jr!(
        execute_0x30_not_taken,
        0x30,
        0x05,
        |cpu: &mut Cpu| cpu.registers.set_c(true),
        8,
        0x0101
    );
    test_jr!(
        execute_0x38_taken,
        0x38,
        0x05,
        |cpu: &mut Cpu| cpu.registers.set_c(true),
        12,
        0x0106
    );
    test_jr!(
        execute_0x38_not_taken,
        0x38,
        0x05,
        |cpu: &mut Cpu| cpu.registers.set_c(false),
        8,
        0x0101
    );

    test_jp!(
        execute_0xC2_taken,
        0xC2,
        |cpu: &mut Cpu| cpu.registers.set_z(false),
        16,
        0x1234
    );
    test_jp!(
        execute_0xC2_not_taken,
        0xC2,
        |cpu: &mut Cpu| cpu.registers.set_z(true),
        12,
        0x0102
    );
    test_jp!(execute_0xC3, 0xC3, |_| {}, 16, 0x1234);
    test_jp!(
        execute_0xCA_taken,
        0xCA,
        |cpu: &mut Cpu| cpu.registers.set_z(true),
        16,
        0x1234
    );
    test_jp!(
        execute_0xCA_not_taken,
        0xCA,
        |cpu: &mut Cpu| cpu.registers.set_z(false),
        12,
        0x0102
    );
    test_jp!(
        execute_0xD2_taken,
        0xD2,
        |cpu: &mut Cpu| cpu.registers.set_c(false),
        16,
        0x1234
    );
    test_jp!(
        execute_0xD2_not_taken,
        0xD2,
        |cpu: &mut Cpu| cpu.registers.set_c(true),
        12,
        0x0102
    );
    test_jp!(
        execute_0xDA_taken,
        0xDA,
        |cpu: &mut Cpu| cpu.registers.set_c(true),
        16,
        0x1234
    );
    test_jp!(
        execute_0xDA_not_taken,
        0xDA,
        |cpu: &mut Cpu| cpu.registers.set_c(false),
        12,
        0x0102
    );

    #[test]
    fn execute_0xE9() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_hl(0x1234);

        let cycles = cpu.execute(0xE9, &mut bus);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.pc, 0x1234);
    }

    test_call!(
        execute_0xC4_taken,
        0xC4,
        |cpu: &mut Cpu| cpu.registers.set_z(false),
        24,
        0x1234,
        0xFFFC,
        true
    );
    test_call!(
        execute_0xC4_not_taken,
        0xC4,
        |cpu: &mut Cpu| cpu.registers.set_z(true),
        12,
        0x0102,
        0xFFFE,
        false
    );
    test_call!(
        execute_0xCC_taken,
        0xCC,
        |cpu: &mut Cpu| cpu.registers.set_z(true),
        24,
        0x1234,
        0xFFFC,
        true
    );
    test_call!(
        execute_0xCC_not_taken,
        0xCC,
        |cpu: &mut Cpu| cpu.registers.set_z(false),
        12,
        0x0102,
        0xFFFE,
        false
    );
    test_call!(execute_0xCD, 0xCD, |_| {}, 24, 0x1234, 0xFFFC, true);
    test_call!(
        execute_0xD4_taken,
        0xD4,
        |cpu: &mut Cpu| cpu.registers.set_c(false),
        24,
        0x1234,
        0xFFFC,
        true
    );
    test_call!(
        execute_0xD4_not_taken,
        0xD4,
        |cpu: &mut Cpu| cpu.registers.set_c(true),
        12,
        0x0102,
        0xFFFE,
        false
    );
    test_call!(
        execute_0xDC_taken,
        0xDC,
        |cpu: &mut Cpu| cpu.registers.set_c(true),
        24,
        0x1234,
        0xFFFC,
        true
    );
    test_call!(
        execute_0xDC_not_taken,
        0xDC,
        |cpu: &mut Cpu| cpu.registers.set_c(false),
        12,
        0x0102,
        0xFFFE,
        false
    );

    test_ret!(
        execute_0xC0_taken,
        0xC0,
        |cpu: &mut Cpu| cpu.registers.set_z(false),
        20,
        0x1234,
        0xFFFE
    );
    test_ret!(
        execute_0xC0_not_taken,
        0xC0,
        |cpu: &mut Cpu| cpu.registers.set_z(true),
        8,
        0x0100,
        0xFFFC
    );
    test_ret!(
        execute_0xC8_taken,
        0xC8,
        |cpu: &mut Cpu| cpu.registers.set_z(true),
        20,
        0x1234,
        0xFFFE
    );
    test_ret!(
        execute_0xC8_not_taken,
        0xC8,
        |cpu: &mut Cpu| cpu.registers.set_z(false),
        8,
        0x0100,
        0xFFFC
    );
    test_ret!(execute_0xC9, 0xC9, |_| {}, 16, 0x1234, 0xFFFE);
    test_ret!(
        execute_0xD0_taken,
        0xD0,
        |cpu: &mut Cpu| cpu.registers.set_c(false),
        20,
        0x1234,
        0xFFFE
    );
    test_ret!(
        execute_0xD0_not_taken,
        0xD0,
        |cpu: &mut Cpu| cpu.registers.set_c(true),
        8,
        0x0100,
        0xFFFC
    );
    test_ret!(
        execute_0xD8_taken,
        0xD8,
        |cpu: &mut Cpu| cpu.registers.set_c(true),
        20,
        0x1234,
        0xFFFE
    );
    test_ret!(
        execute_0xD8_not_taken,
        0xD8,
        |cpu: &mut Cpu| cpu.registers.set_c(false),
        8,
        0x0100,
        0xFFFC
    );

    #[test]
    fn execute_0xD9() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.sp = 0xFFFC;
        write_return_address(&mut bus, 0xFFFC, 0x1234);

        let cycles = cpu.execute(0xD9, &mut bus);

        assert_eq!(cycles, 16);
        assert_eq!(cpu.registers.pc, 0x1234);
        assert_eq!(cpu.registers.sp, 0xFFFE);
        assert!(cpu.ime);
    }

    test_rst!(execute_0xC7, 0xC7, 0x00);
    test_rst!(execute_0xCF, 0xCF, 0x08);
    test_rst!(execute_0xD7, 0xD7, 0x10);
    test_rst!(execute_0xDF, 0xDF, 0x18);
    test_rst!(execute_0xE7, 0xE7, 0x20);
    test_rst!(execute_0xEF, 0xEF, 0x28);
    test_rst!(execute_0xF7, 0xF7, 0x30);
    test_rst!(execute_0xFF, 0xFF, 0x38);
}
