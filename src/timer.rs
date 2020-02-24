use crate::memory::Memory;

pub static PRESCALER: [u32; 4] =
[
    1,
    64,
    256,
    1024,
];

pub struct Timers
{
    pub timer: Vec<Timer>
}

#[derive(Clone, Debug)]
pub struct Timer
{
    pub index    : usize,       // 0 - 3
    pub counter  : u32,         // Accumulater counts, should be < prescaler

    pub data     : *mut u16,    // Pointer to ioram timer data

    pub prescaler: u32,         // 1, 64, 256, 1024
    pub irq_f    : bool,        // Interrupt on overflow
    pub cascade_f: bool,        // Cascade flag
    pub enable   : bool,        // Enable flag
}

impl Timers
{
    pub fn new(memory: &mut Memory) -> Self
    {
        let mut t = Self
        {
            timer: vec![Timer::new(); 4],
        };

        for i in 0..4
        {
            t.timer[i].index = i;
            t.timer[i].data  = memory.get_timer_data(i);
        }

        t
    }

    pub fn update(&mut self, value: u32, memory: &mut Memory)
    {
        for i in 0..4
        {
            memory.update_tmcnt(&mut self.timer[i]);

            self.timer[i].increment_counter(value);
        }
    }
}

impl Timer
{
    pub fn new() -> Self
    {
        Self
        {
            index    : 0,
            counter  : 0,
            data     : 0 as *mut u16,
            prescaler: 0,
            irq_f    : false,
            cascade_f: false,
            enable   : false,
        }
    }

    pub fn increment_counter(&mut self, ticks: u32)
    {
        if self.enable
        {
            // dbg!(&self);
            // unsafe {println!("{}", *self.data);}
            
            self.counter += ticks;

            let value = self.counter / self.prescaler;

            self.increment_data(value);

            self.counter = self.counter % self.prescaler;
        }
    }

    pub fn increment_data(&mut self, value: u32)
    {
        unsafe
        {
            *self.data += value as u16;
        }
    }
}