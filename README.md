# GameBar
A Game Boy Advance emulator written in Rust

## Running
```
cargo run --release -- <rom.gba>
```
The emulator will crash very often if you build it in debug mode due to integer overflow checks. Running in release mode gives better performance and avoid these sorts of problems.

## TODO
- EEPROM IO
