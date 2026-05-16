# Rust Gameboy Emulator

Curently a work in progress, building through the following resources:

[Full Specs](https://gbdev.io/pandocs/About.html)

[PPU Testing](https://github.com/mattcurrie/dmg-acid2)

[OPCODE Reference](https://izik1.github.io/gbops/)

I'm planning to build out the emulator core in rust, compile to web assembly, and run it in browser.

and at some point, I'd like to upgrade to the Gameboy Color too. Though, it's not as documented...

We'll see how it goes...

## Current Progress

Non-color mode games can be displayed, to run, for the repo and at the repo root run

`cargo run --release -p gui /path/to/game`

### Currently Working On:

Games can run, but there is no functionality for user input. I need to implement joypad interrupts.

### Memory Map / Bus

I've now implemented the following parts of the memory map / IO:

- Cartridge (without MBC controllers)
- Serial Port
- PPU
- VRAM (as part of PPU)
- OAM (as part of PPU)
- Timer (and its related registers)

I've yet to implement the following (They are currently 'stubs' and implemented as a static array)

- ERAM
- WRAM
- IO
- HRAM

### CPU

All ~500 instructions are implemented, and I am currently going through the Blargg test suite

[Test Suite](https://github.com/retrio/gb-test-roms/tree/c240dd7d700e5c0b00a7bbba52b53e4ee67b5f15)

Currently, only interrupts are not fully implemented. I need to finish the PPU, Audio, and IO first.
