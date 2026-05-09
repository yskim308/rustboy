# Rust Gameboy Emulator

Curently a work in progress, building through the following resources:

[Full Specs](http://bgb.bircd.org/pandocs.htm])

[PPU Testing](https://github.com/mattcurrie/dmg-acid2)

[OPCODE Reference](https://izik1.github.io/gbops/)

I'm planning to build out the emulator core in rust, compile to web assembly, and run it in browser.

and at some point, I'd like to upgrade to the Gameboy Color too. Though, it's not as documented...

We'll see how it goes...

## Current Progress

### Memory Map / Bus

I've now implemented the following parts of the memory map:

- Cartridge (without MBC controllers)
- Serial Port

I've yet to implement the following (They are currently 'stubs' and implemented as a static array)

- VRAM
- ERAM
- WRAM
- OAM
- IO
- HRAM

### CPU

All ~500 instructions are implemented, and I am currently going through the Blargg test suite

[Test Suite](https://github.com/retrio/gb-test-roms/tree/c240dd7d700e5c0b00a7bbba52b53e4ee67b5f15)

Once the CPU Instruction suite passes, I will move on to implementing the rest of the hardware with synchronization.
