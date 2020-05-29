use crate::util::*;
use crate::ppu::PPU;
use crate::ppu::background::Background;
use crate::ppu::background::DIMENSION;
use crate::ppu::window::Window;

impl PPU
{
    pub fn get_dispcnt(&self) -> u16
    {
        self.dispcnt
    }

    pub fn set_dispcnt(&mut self, value: u16)
    {
        self.dispcnt    = value;
        self.mode       = value.bits(2, 0);
        self.flip       = value.bit(4);
        self.sequential = value.bit(6);
        self.fblank     = value.bit(7);
    }

    pub fn get_dispstat(&self) -> u16
    {
        self.dispstat
    }

    pub fn set_dispstat(&mut self, value: u16)
    {
        self.dispstat = value;
    }

    pub fn get_vcount(&self) -> u16
    {
        self.vcount
    }
}

impl Background
{
    pub fn get_control(&self) -> u16
    {
        self.bgcnt
    }

    pub fn set_control(&mut self, value: u16)
    {
        self.bgcnt     = value;
        self.priority  = value.bits(1, 0);
        self.tile_b    = value.bits(3, 2);
        self.mosaic_f  = value.bit(6);
        self.palette_f = value.bit(7);
        self.map_b     = value.bits(12, 8);
        self.wrap_f    = value.bit(13);
        self.size_r    = value.bits(15, 14);

        self.width  = DIMENSION[self.affine_f as usize][self.size_r as usize].0;
        self.height = DIMENSION[self.affine_f as usize][self.size_r as usize].1;
    }

    pub fn set_hofs(&mut self, value: u16)
    {
        self.hscroll = value;
    }

    pub fn set_vofs(&mut self, value: u16)
    {
        self.vscroll = value;
    }

    pub fn set_pa(&mut self, value: u16)
    {
        self.matrix.0 = value as i16 as i32;
    }

    pub fn set_pb(&mut self, value: u16)
    {
        self.matrix.1 = value as i16 as i32;
    }

    pub fn set_pc(&mut self, value: u16)
    {
        self.matrix.2 = value as i16 as i32;
    }

    pub fn set_pd(&mut self, value: u16)
    {
        self.matrix.3 = value as i16 as i32;
    }

    pub fn set_x_l(&mut self, value: u16)
    {
        self.coord.0 = ((self.coord.0 as u32) & 0xffff0000) as i32;
        self.coord.0 |= value as i32;
    }

    pub fn set_x_h(&mut self, value: u16)
    {
        self.coord.0 &= 0x0000ffff;
        self.coord.0 |= ((value as u32) << 16) as i32;
    }
    pub fn set_y_l(&mut self, value: u16)
    {
        self.coord.1 = ((self.coord.0 as u32) & 0xffff0000) as i32;
        self.coord.1 |= value as i32;
    }
    pub fn set_y_h(&mut self, value: u16)
    {
        self.coord.1 &= 0x0000ffff;
        self.coord.1 |= ((value as u32) << 16) as i32;
    }
}

impl Window
{
    pub fn get_win0h(&self) -> u16
    {
        self.winh[0]
    }

    pub fn get_win1h(&self) -> u16
    {
        self.winh[1]
    }

    pub fn get_win0v(&self) -> u16
    {
        self.winv[0]
    }

    pub fn get_win1v(&self) -> u16
    {
        self.winv[1]
    }

    pub fn get_winin(&self) -> u16
    {
        self.winin
    }

    pub fn get_winout(&self) -> u16
    {
        self.winout
    }

    pub fn set_win0h(&mut self, value: u16)
    {
        self.winh[0] = value;
    }

    pub fn set_win1h(&mut self, value: u16)
    {
        self.winh[1] = value;
    }

    pub fn set_win0v(&mut self, value: u16)
    {
        self.winv[0] = value;
    }

    pub fn set_win1v(&mut self, value: u16)
    {
        self.winv[1] = value;
    }

    pub fn set_winin(&mut self, value: u16)
    {
        self.winin = value;
    }

    pub fn set_winout(&mut self, value: u16)
    {
        self.winout = value;
    }
}