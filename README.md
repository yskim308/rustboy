# Rust Gameboy Emulator

Curently a work in progress, building through the following resources:

[Full Specs](https://gbdev.io/pandocs/About.html)

[PPU Testing](https://github.com/mattcurrie/dmg-acid2)

[OPCODE Reference](https://izik1.github.io/gbops/)

I'm planning to build out the emulator core in rust, compile to web assembly, and run it in browser.

and at some point, I'd like to upgrade to the Gameboy Color too. Though, it's not as documented...

We'll see how it goes...

## Current Progress

### Currently Working On:

Right now, I'm currently working on the PPU in DMG (Non color) mode.

The actual Gameboy hardware does FIFO pushing with each pixel, but for now, I'm implementing it line by line.

It should work for ~90% of games. I'll come back and do 100% accurate emulation once the other areas are implemented.

### Memory Map / Bus

I've now implemented the following parts of the memory map:

- Cartridge (without MBC controllers)
- Serial Port
- VRAM (as part of PPU)
- OAM (as part of PPU)

I've yet to implement the following (They are currently 'stubs' and implemented as a static array)

- ERAM
- WRAM
- IO
- HRAM

### CPU

All ~500 instructions are implemented, and I am currently going through the Blargg test suite

[Test Suite](https://github.com/retrio/gb-test-roms/tree/c240dd7d700e5c0b00a7bbba52b53e4ee67b5f15)

Currently, only interrupts are not fully implemented. I need to finish the PPU, Audio, and IO first.
