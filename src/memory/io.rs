use super::Memory;

impl Memory
{
    #[inline]
    pub fn load_reg(&self, index: usize) -> u16
    {
        unsafe
        {
            let ptr = self.ioram.as_ptr() as *const u16;

            *ptr.add(index / 2)
        }
    }

    #[inline]
    pub fn store_reg(&mut self, index: usize, value: u16)
    {
        unsafe
        {
            let ptr = self.ioram.as_mut_ptr() as *mut u16;

            *ptr.add(index / 2) = value;
        }
    }

    pub fn get_dispcnt(&self) -> u16
    {
        self.load_reg(0x00)
    }

    pub fn get_dispstat(&self) -> u16
    {
        self.load_reg(0x04)
    }

    pub fn set_vblank_flag(&mut self, value: bool)
    {
        let mut dispstat = self.load_reg(0x04);

        if value { dispstat |= 0b01 } else { dispstat &= !0b01 }

        self.store_reg(0x04, dispstat);
    }

    pub fn set_hblank_flag(&mut self, value: bool)
    {
        let mut dispstat = self.load_reg(0x04);

        if value { dispstat |= 0b10 } else { dispstat &= !0b10 }

        self.store_reg(0x04, dispstat);
    }

    pub fn get_vcount(&self) -> u16
    {
        self.load_reg(0x06)
    }

    pub fn inc_vcount(&mut self)
    {
        let vcount = self.get_vcount();
        self.store_reg(0x06, (vcount + 1) as u16);
    }

    pub fn clr_vcount(&mut self)
    {
        self.store_reg(0x06, 0);
    }

    pub fn get_bgcnt(&self, index: usize) -> u16
    {
        self.load_reg(0x08 + index * 2)
    }

    pub fn get_bghofs(&self, index: usize) -> u16
    {
        self.load_reg(0x10 + index * 4)
    }

    pub fn get_bgvofs(&self, index: usize) -> u16
    {
        self.load_reg(0x12 + index * 4)
    }
}

