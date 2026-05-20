# Rust Gameboy Emulator

Curently a work in progress, building through the following resources:

[Full Specs](https://gbdev.io/pandocs/About.html)

[PPU Testing](https://github.com/mattcurrie/dmg-acid2)

[OPCODE Reference](https://izik1.github.io/gbops/)

I'm planning to build out the emulator core in rust, compile to web assembly, and run it in browser.

and at some point, I'd like to upgrade to the Gameboy Color too. Though, it's not as documented...

We'll see how it goes...

## Current Progress

The very basic prototype works!!! To run, (after installing Cargo), run the following command at the project root

```bash
cargo run --release -p gui /path/to/game
```

<img width="356" height="331" alt="image" src="https://github.com/user-attachments/assets/4be970d4-e977-4045-82a0-77759ea9bded" />
<img width="331" height="318" alt="image" src="https://github.com/user-attachments/assets/3abf232f-ee8d-4657-8e15-fdc47c51b410" />
<img width="328" height="321" alt="image" src="https://github.com/user-attachments/assets/ee2514e5-3168-4890-8121-544cbe687961" />
<img width="323" height="301" alt="image" src="https://github.com/user-attachments/assets/e9fbf624-1bc9-44fb-a667-3a0d6638d9a4" />




### CPU

All ~500 instructions are implemented, abd the entire Blargg Test suite passes. 

[Test Suite](https://github.com/retrio/gb-test-roms/tree/c240dd7d700e5c0b00a7bbba52b53e4ee67b5f15)

I am now working on the other parts of the console. 

### Currently Working On:

Basic games work with no issues. Next up is 

0. ~100% Multi-Bank controller support (currently MBC1 and MBC3 are implemented)
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
- ERAM (as part of the Cartridge)
- WRAM
- IO
- HRAM


