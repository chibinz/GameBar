use super::Memory;

impl Memory
{
    /// Return a row of 4-bit tile data as a word
    #[inline]
    pub fn tile_row32(&self, index: u32, tile_number: u32, row: u32) -> u32
    {
        unsafe
        {
            let ptr = self.vram.as_ptr() as *const u32;

            *ptr.add((index * 0x4000 / 4 + tile_number * 8 + row) as usize)
        }
    }

    /// Return tile map entry
    #[inline]
    pub fn tile_map(&self, index: u32, y: u32, x: u32) -> u16
    {
        unsafe
        {
            let ptr = self.vram.as_ptr() as *const u16;

            *ptr.add((index * 0x800 / 2 + y * 32 + x) as usize)
        }
    }
}