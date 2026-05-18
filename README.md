# Rust Gameboy Emulator

Curently a work in progress, building through the following resources:

[Full Specs](https://gbdev.io/pandocs/About.html)

[PPU Testing](https://github.com/mattcurrie/dmg-acid2)

[OPCODE Reference](https://izik1.github.io/gbops/)

I'm planning to build out the emulator core in rust, compile to web assembly, and run it in browser.

and at some point, I'd like to upgrade to the Gameboy Color too. Though, it's not as documented...

We'll see how it goes...

## Current Progress

The very basic prototype works!!! To run, (after installing Cargo), run the following command 

`cargo run --release -p gui /path/to/game`

<img width="483" height="413" alt="image" src="https://github.com/user-attachments/assets/77b389d0-0313-42b6-aff9-d70e4c9d37e9" />


### Currently Working On:

Basic games work with no issues. Next up is 

1. Audio (APU)
2. CGB (color mode for gameboy color)
3. FIFO / DMA based PPU processing for better accuracy on more complex games
4. TUI and Web (WASM) based rendering 

### Memory Map / Bus

I've now implemented the following parts of the memory map / IO:

- Cartridge (without MBC controllers)
- Serial Port
- PPU
- VRAM (as part of PPU)
- OAM (as part of PPU)
- Timer (and its related registers)
   ERAM
- WRAM
- IO
- HRAM

### CPU

All ~500 instructions are implemented, and I am currently going through the Blargg test suite

[Test Suite](https://github.com/retrio/gb-test-roms/tree/c240dd7d700e5c0b00a7bbba52b53e4ee67b5f15)
