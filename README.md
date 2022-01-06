# GameBar
A Game Boy Advance emulator written in Rust

GameBar is an experimental GBA emulator that I've started development during 2019's summer. It is mostfully feature complete, with a ARM7TDMI cpu interpreter, and ppu, timer, dma, interrupt controller emulation. It is able to run quite a few popular commercial game titles， such as Pokemon Emerald, Fire Emblem, and many more.

## Running
Please place the bios file in `rom/gba_bios.bin`.
```
cargo run --release -- rom/emerald.gba
```
The emulator will crash very often if you build it in debug mode due to integer overflow checks. Running in release mode gives better performance and avoid these sorts of problems.
