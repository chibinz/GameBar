use super::GBus;

impl GBus {
    pub fn access_timing(address: usize, size: u32) -> i32 {
        // Do not distinguish between sequential and non sequential cycles
        let timing = match Self::region(address) {
            0x00 => [1, 1, 1],
            0x02 => [3, 3, 6],
            0x03 => [1, 1, 1],
            0x04 => [1, 1, 1],
            0x05 => [1, 1, 2],
            0x06 => [1, 1, 2],
            0x07 => [1, 1, 1],
            0x08..=0x0d => [5, 5, 8],
            // Open bus access timing
            _ => [1, 1, 1],
        };

        timing[size as usize]
    }
}
