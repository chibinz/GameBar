# GameBar
A Game Boy Advance emulator written in Rust

GameBar is an experimental GBA emulator that I've started development during 2019's summer. It is mostfully feature complete, with a ARM7TDMI cpu interpreter, and ppu, timer, dma, interrupt controller emulation. It is able to run quite a few popular commercial game titles， such as Pokemon Emerald, Fire Emblem, and many more.

## Running
Please place the bios file in `rom/gba_bios.bin`.
```
cargo run --release -- <rom>
```
The emulator will crash very often if you build it in debug mode due to integer overflow checks. Running in release mode gives better performance and avoid these sorts of problems.

## Credits
- [jsmolka/eggvance](https://github.com/jsmolka/eggvance), pretty clean implementation!
- [jsmolka/gba-tests](https://github.com/jsmolka/gba-tests), a very comprehensive cpu test suite
- [mgba-emu/suite](https://github.com/mgba-emu/suite), very good test suite for testing peripherals
- [DenSinH/FuzzARM](https://github.com/DenSinH/FuzzARM), test rom via fuzzing!
- [GBATEK](https://problemkaputt.de/gbatek.htm), goto reference for GBA emulation
- [TONC](https://www.coranac.com/tonc), explains how peripheral hardware works under the hood. A more approachabl text compared to GBATEK.
- And also thanks to all the folks on the EmuDev Discord for their help and support!
