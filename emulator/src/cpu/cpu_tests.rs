use crate::bus::{Bus, cartridge::Cartridge};

use super::Cpu;

fn test_bus(bytes_at_pc: [u8; 2]) -> Bus {
    let mut rom = vec![0; 0x8000];
    rom[0x0100] = bytes_at_pc[0];
    rom[0x0101] = bytes_at_pc[1];
    Bus::new(Cartridge::new(rom))
}

#[test]
fn execute_0xC3() {
    let mut cpu = Cpu::new();
    let mut bus = test_bus([0x34, 0x12]);

    let cycles = cpu.execute(0xC3, &mut bus);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.registers.pc, 0x1234);
}
