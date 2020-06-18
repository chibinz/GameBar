use super::Memory;

impl Memory
{
    pub fn cpu_access_timing(address: u32, size: u32) -> i32
    {
        // Do not distinguish between sequential and non sequential cycles
        let timing =
        [
            // 8, 16, 32 bit access
            [1, 1, 1], // BIOS
            [0, 0, 0],
            [3, 3, 6], // EWRAM
            [1, 1, 1], // IWRAM
            [1, 1, 1], // IOREG
            [1, 1, 2], // PARAM
            [1, 1, 2], // VRAM
            [1, 1, 1], // OAM
            [5, 5, 8], // ROM / FLASH
            [5, 5, 8],
            [5, 5, 8],
            [5, 5, 8],
            [5, 5, 8],
            [5, 5, 8],
            [5, 0, 0], // SRAM
        ];

        timing[Self::region(address) as usize][size as usize]
    }
}