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

    macro_rules! test_ld_r_u8 {
        ($name:ident, $opcode:expr, $register:ident, $value:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[$value]);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 8);
                assert_eq!(cpu.registers.$register, $value);
            }
        };
    }

    macro_rules! test_ld_rr_u16 {
        ($name:ident, $opcode:expr, $getter:ident) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[0x34, 0x12]);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 12);
                assert_eq!(cpu.registers.$getter(), 0x1234);
            }
        };
    }

    macro_rules! test_ld_r_r {
        ($name:ident, $opcode:expr, $dst:ident, $src:ident, $value:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup(&[]);
                cpu.registers.$src = $value;

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 4);
                assert_eq!(cpu.registers.$dst, $value);
                assert_eq!(cpu.registers.$src, $value);
            }
        };
    }

    test_ld_rr_u16!(execute_0x01, 0x01, get_bc);
    test_ld_r_u8!(execute_0x06, 0x06, b, 0x12);
    test_ld_r_u8!(execute_0x0E, 0x0E, c, 0x12);
    test_ld_rr_u16!(execute_0x11, 0x11, get_de);
    test_ld_r_u8!(execute_0x16, 0x16, d, 0x12);
    test_ld_r_u8!(execute_0x1E, 0x1E, e, 0x12);
    test_ld_rr_u16!(execute_0x21, 0x21, get_hl);
    test_ld_r_u8!(execute_0x26, 0x26, h, 0x12);
    test_ld_r_u8!(execute_0x2E, 0x2E, l, 0x12);

    #[test]
    fn execute_0x31() {
        let (mut cpu, mut bus) = setup(&[0x34, 0x12]);

        let cycles = cpu.execute(0x31, &mut bus);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.sp, 0x1234);
    }

    test_ld_r_u8!(execute_0x3E, 0x3E, a, 0x12);

    test_ld_r_r!(execute_0x40, 0x40, b, b, 0x12);
    test_ld_r_r!(execute_0x41, 0x41, b, c, 0x12);
    test_ld_r_r!(execute_0x42, 0x42, b, d, 0x12);
    test_ld_r_r!(execute_0x43, 0x43, b, e, 0x12);
    test_ld_r_r!(execute_0x44, 0x44, b, h, 0x12);
    test_ld_r_r!(execute_0x45, 0x45, b, l, 0x12);
    test_ld_r_r!(execute_0x47, 0x47, b, a, 0x12);

    test_ld_r_r!(execute_0x48, 0x48, c, b, 0x12);
    test_ld_r_r!(execute_0x49, 0x49, c, c, 0x12);
    test_ld_r_r!(execute_0x4A, 0x4A, c, d, 0x12);
    test_ld_r_r!(execute_0x4B, 0x4B, c, e, 0x12);
    test_ld_r_r!(execute_0x4C, 0x4C, c, h, 0x12);
    test_ld_r_r!(execute_0x4D, 0x4D, c, l, 0x12);
    test_ld_r_r!(execute_0x4F, 0x4F, c, a, 0x12);

    test_ld_r_r!(execute_0x50, 0x50, d, b, 0x12);
    test_ld_r_r!(execute_0x51, 0x51, d, c, 0x12);
    test_ld_r_r!(execute_0x52, 0x52, d, d, 0x12);
    test_ld_r_r!(execute_0x53, 0x53, d, e, 0x12);
    test_ld_r_r!(execute_0x54, 0x54, d, h, 0x12);
    test_ld_r_r!(execute_0x55, 0x55, d, l, 0x12);
    test_ld_r_r!(execute_0x57, 0x57, d, a, 0x12);

    test_ld_r_r!(execute_0x58, 0x58, e, b, 0x12);
    test_ld_r_r!(execute_0x59, 0x59, e, c, 0x12);
    test_ld_r_r!(execute_0x5A, 0x5A, e, d, 0x12);
    test_ld_r_r!(execute_0x5B, 0x5B, e, e, 0x12);
    test_ld_r_r!(execute_0x5C, 0x5C, e, h, 0x12);
    test_ld_r_r!(execute_0x5D, 0x5D, e, l, 0x12);
    test_ld_r_r!(execute_0x5F, 0x5F, e, a, 0x12);

    test_ld_r_r!(execute_0x60, 0x60, h, b, 0x12);
    test_ld_r_r!(execute_0x61, 0x61, h, c, 0x12);
    test_ld_r_r!(execute_0x62, 0x62, h, d, 0x12);
    test_ld_r_r!(execute_0x63, 0x63, h, e, 0x12);
    test_ld_r_r!(execute_0x64, 0x64, h, h, 0x12);
    test_ld_r_r!(execute_0x65, 0x65, h, l, 0x12);
    test_ld_r_r!(execute_0x67, 0x67, h, a, 0x12);

    test_ld_r_r!(execute_0x68, 0x68, l, b, 0x12);
    test_ld_r_r!(execute_0x69, 0x69, l, c, 0x12);
    test_ld_r_r!(execute_0x6A, 0x6A, l, d, 0x12);
    test_ld_r_r!(execute_0x6B, 0x6B, l, e, 0x12);
    test_ld_r_r!(execute_0x6C, 0x6C, l, h, 0x12);
    test_ld_r_r!(execute_0x6D, 0x6D, l, l, 0x12);
    test_ld_r_r!(execute_0x6F, 0x6F, l, a, 0x12);

    test_ld_r_r!(execute_0x78, 0x78, a, b, 0x12);
    test_ld_r_r!(execute_0x79, 0x79, a, c, 0x12);
    test_ld_r_r!(execute_0x7A, 0x7A, a, d, 0x12);
    test_ld_r_r!(execute_0x7B, 0x7B, a, e, 0x12);
    test_ld_r_r!(execute_0x7C, 0x7C, a, h, 0x12);
    test_ld_r_r!(execute_0x7D, 0x7D, a, l, 0x12);
    test_ld_r_r!(execute_0x7F, 0x7F, a, a, 0x12);

    #[test]
    fn execute_0xF9() {
        let (mut cpu, mut bus) = setup(&[]);
        cpu.registers.set_hl(0x1234);

        let cycles = cpu.execute(0xF9, &mut bus);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.sp, 0x1234);
    }

    #[test]
    fn execute_0xC3() {
        let (mut cpu, mut bus) = setup(&[0x34, 0x12]);

        let cycles = cpu.execute(0xC3, &mut bus);

        assert_eq!(cycles, 16);
        assert_eq!(cpu.registers.pc, 0x1234);
    }
}
