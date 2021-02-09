# GameBar
A Game Boy Advance emulator written in Rust

## Running
```
cargo run --release -- <rom.gba>
```
The emulator will crash very often if you build it in debug mode due to integer overflow checks. Running in release mode gives better performance and avoid these sorts of problems.

## TODO
- EEPROM IO

## Tile caching
Whether background or sprite tiles, they are always contiguous in memory. If it is a 4bpp pixel, it will be 64 / 2 = 32 bytes. If it is a 8 bpp pixel, it will occupy 64 bytes. An issue with caching tiles is that background maps and object attributes keep references to the vram where the tile data is actually stored, and tiles are once again indirect references to pixels in the palette ram. So a cached tile should be rerendered if a write to attributes, tile data, and corresponding palette memory occurs. Recalculating the hash for the whole tile data (32 bytes) or palette ram (256 * 2) bytes every single scanline is just too expansive. There needs to be some way of knowing whether the tile data or palette ram is dirty when a write occurs. A solution that I have in mind is to eagerly calculate the hash of tile data and palette memory upon write, since writes are infrequent and ppu reads are hot. (Interpretation vs Representation). Implementationwise, this would require placing hash value alongside vram and palette ram. When a write to vram happens, calculate the byte index rounded to the nearest multiple of 32. Hash the consecutive 32 bytes beginning with the index. And store the value in the hash value array. For instance, if vram[37] is written, hash vram[32..64], store the value in vram_hash[1]. Since we're referencing contiguous memory, the cost shouldn't be very large. Similar strategy could be applied to palette ram, except that we need special case handling for 8bpp tiles. 16 entries for 16 rows, and 1 entry for hash of 16 hashes as global palette ram hash.

### Function
```Rust
#[cached]
get_tile(hflip: bool, vflip: bool, tile_hash1: u64, tile_hash2: u64, palette: u64) -> &[u16; 8 * 8] {
    // Render tile
}
```

### Size
- vram_hash 96 / 32 * 8 = 24 kbytes
- param_hash (16 * 16 * 2 / 32 * 8 + 1) * 2 = 258 bytes
- hash table 1024 entries, 1024 * 8 * 8 * 2 = 128 kbytes
- frame buffer 240 * 160 * 2 = 75 kbytes

### Assumptions
- Write to palette ram, and vram is relatively infrequent
- Hash table look ups are much faster than rerendering

### Issues
- Hash table is good for unbounded cache, need a way to delete cold entries.
