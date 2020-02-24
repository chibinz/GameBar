use crate::memory::Memory;

pub struct DMA
{
    pub channel: Vec<DMAChannel>
}

#[derive(Clone, Debug)]
pub struct DMAChannel
{
    pub index: usize,       // DMA index 0 - 3
    pub src  : u32,         // Internal register of current source address
    pub dst  : u32,         // Internal register of current destination address
    pub count: u32,         // Number of word / halfword to be copied
    
    // 0b00 - increment
    // 0b01 - decrement
    // 0b10 - fixed
    // 0b11 - increment / reload after transfer (Prohibited for srccnt)
    pub srccnt  : u32,      // Source addrss control
    pub dstcnt  : u32,      // Destination address control
    pub repeat_f: bool,     // Repeat flag for HBlank / VBlank DMA
    pub word_f  : bool,     // 0 - halfword, 1 - word
    pub drq_f   : bool,     // Gamepak DRQ
    pub start   : u32,      // Start time
    pub irq_f   : bool,     // IRQ upon completion
    pub enable  : bool,     // Enable flag
}

impl DMA
{
    pub fn new() -> Self
    {
        let mut d = Self
        {
            channel: vec![DMAChannel::new(); 4],
        };

        for i in 0..4
        {
            d.channel[i].index = i;
        }

        d
    }

    pub fn request(&mut self, memory: &mut Memory)
    {
        for i in 0..4
        {
            memory.update_dma(&mut self.channel[i]);

            self.channel[i].transfer(memory);
        }
    }
}

impl DMAChannel
{
    pub fn new() -> Self
    {
        Self
        {
            index   : 0,
            src     : 0,
            dst     : 0,
            count   : 0,
            dstcnt  : 0,
            srccnt  : 0,
            repeat_f: false,
            word_f  : false,
            drq_f   : false,
            start   : 0,
            irq_f   : false,
            enable  : false,
        }
    }

    pub fn transfer(&mut self, memory: &mut Memory)
    {
        if !self.enable {return}

        self.read_ioreg(memory);

        if self.word_f
        {
            self.transfer32(memory);
        }
        else
        {
            self.transfer16(memory);
        }

        self.write_ioreg(memory);

        if !self.repeat_f
        {
            self.enable = false;
            memory.clr_dma(self.index);
        }
    }

    pub fn transfer16(&mut self, memory: &mut Memory)
    {
        let srcinc = Self::get_increment(self.word_f, self.srccnt);
        let dstinc = Self::get_increment(self.word_f, self.dstcnt); 
        
        for _ in 0..self.count
        {
            memory.store16(self.dst, memory.load16(self.src));

            // Incrment internal register
            self.src = self.src.wrapping_add(srcinc);
            self.dst = self.dst.wrapping_add(dstinc);
        }
    }

    pub fn transfer32(&mut self, memory: &mut Memory)
    {
        let srcinc = Self::get_increment(self.word_f, self.srccnt);
        let dstinc = Self::get_increment(self.word_f, self.dstcnt); 
        
        for _ in 0..self.count
        {
            memory.store32(self.dst, memory.load32(self.src));

            // Increment internal register
            self.src = self.src.wrapping_add(srcinc);
            self.dst = self.dst.wrapping_add(dstinc);
        }
    }

    /// Read internal register from ioram
    pub fn read_ioreg(&mut self, memory: &Memory)
    {
        memory.update_dmadad(self);
        memory.update_dmasad(self);
    }

    /// Write internal register into ioram
    pub fn write_ioreg(&self, memory: &mut Memory)
    {
        let sad = 0x040000B0 + (self.index as u32 * 0xc);
        let dad = 0x040000B4 + (self.index as u32 * 0xc);
        
        memory.store32(sad, self.src);
        if self.dstcnt != 3 {memory.store32(dad, self.dst)}
    }

    pub fn get_increment(word_f: bool, cnt: u32) -> u32
    {
        let inc: u32 = if word_f {4} else {2};
        match cnt
        {
            0b00 => inc,
            0b01 => inc.wrapping_neg(),
            0b10 => 0,
            0b11 => inc,
            _    => unreachable!(),
        }
    }
}