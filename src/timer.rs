pub static PRESCALER: [u32; 4] =
[
    1,
    64,
    256,
    1024,
];

#[derive(Clone, Debug)]
pub struct Timer
{
    pub index    : usize,
    pub counter  : u32,

    pub data     : *mut u16,

    pub prescaler: u32,
    pub irq_f    : bool,
    pub cascade_f: bool,
    pub enable   : bool,
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