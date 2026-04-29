#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use crate::{
        bus::{Bus, cartridge::Cartridge},
        cpu::Cpu,
    };

    fn setup() -> (Cpu, Bus) {
        let mut cpu = Cpu::new();
        cpu.registers.pc = 0x0100;

        let rom = vec![0; 0x8000];
        let bus = Bus::new(Cartridge::new(rom));
        (cpu, bus)
    }

    fn write_stack_u16(bus: &mut Bus, sp: u16, value: u16) {
        let [low, high] = value.to_le_bytes();
        bus.write_u8(sp, low);
        bus.write_u8(sp.wrapping_add(1), high);
    }

    fn assert_stack_u16(bus: &Bus, sp: u16, value: u16) {
        let [low, high] = value.to_le_bytes();
        assert_eq!(bus.read_u8(sp), low);
        assert_eq!(bus.read_u8(sp.wrapping_add(1)), high);
    }

    macro_rules! test_pop_r16 {
        ($name:ident, $opcode:expr, $getter:ident, $value:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup();
                cpu.registers.sp = 0xFFFC;
                write_stack_u16(&mut bus, 0xFFFC, $value);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 12);
                assert_eq!(cpu.registers.$getter(), $value);
                assert_eq!(cpu.registers.sp, 0xFFFE);
            }
        };
    }

    macro_rules! test_push_r16 {
        ($name:ident, $opcode:expr, $setter:ident, $getter:ident, $value:expr) => {
            #[test]
            fn $name() {
                let (mut cpu, mut bus) = setup();
                cpu.registers.sp = 0xFFFE;
                cpu.registers.$setter($value);

                let cycles = cpu.execute($opcode, &mut bus);

                assert_eq!(cycles, 16);
                assert_eq!(cpu.registers.$getter(), $value);
                assert_eq!(cpu.registers.sp, 0xFFFC);
                assert_stack_u16(&bus, 0xFFFC, $value);
            }
        };
    }

    test_pop_r16!(execute_0xC1, 0xC1, get_bc, 0x1234);
    test_push_r16!(execute_0xC5, 0xC5, set_bc, get_bc, 0x1234);
    test_pop_r16!(execute_0xD1, 0xD1, get_de, 0x1234);
    test_push_r16!(execute_0xD5, 0xD5, set_de, get_de, 0x1234);
    test_pop_r16!(execute_0xE1, 0xE1, get_hl, 0x1234);
    test_push_r16!(execute_0xE5, 0xE5, set_hl, get_hl, 0x1234);

    #[test]
    fn execute_0xF1() {
        let (mut cpu, mut bus) = setup();
        cpu.registers.sp = 0xFFFC;
        write_stack_u16(&mut bus, 0xFFFC, 0x12F7);

        let cycles = cpu.execute(0xF1, &mut bus);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.get_af(), 0x12F0);
        assert_eq!(cpu.registers.sp, 0xFFFE);
    }

    #[test]
    fn execute_0xF5() {
        let (mut cpu, mut bus) = setup();
        cpu.registers.sp = 0xFFFE;
        cpu.registers.set_af(0x12F7);

        let cycles = cpu.execute(0xF5, &mut bus);

        assert_eq!(cycles, 16);
        assert_eq!(cpu.registers.get_af(), 0x12F0);
        assert_eq!(cpu.registers.sp, 0xFFFC);
        assert_stack_u16(&bus, 0xFFFC, 0x12F0);
    }
}
