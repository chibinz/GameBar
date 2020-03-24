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
    pub reload   : u16,     // Initial value on reload
    pub counter  : u16,     // Timer data
    pub modulo   : u32,     // Leftover (< prescaler)
    pub prescaler: u32,     // 1, 64, 256, 1024
    pub irq_f    : bool,    // Interrupt on overflow
    pub cascade_f: bool,    // Cascade flag
    pub enable   : bool,    // Enable flag
}

impl Timers
{
    pub fn new() -> Self
    {
        Self
        {
            timer: vec![Timer::new(); 4],
        }
    }

    pub fn run(&mut self, value: u32)
    {
        let mut overflow = false;

        for timer in self.timer.iter_mut()
        {
            let increment = if timer.cascade_f {overflow as u32} else {value};

            overflow = timer.increment_counter(increment);
        }
    }
}

impl Timer
{
    pub fn new() -> Self
    {
        Self
        {
            reload   : 0,
            counter  : 0,
            prescaler: 0,
            modulo   : 0,
            irq_f    : false,
            cascade_f: false,
            enable   : false,
        }
    }

    pub fn increment_counter(&mut self, ticks: u32) -> bool
    {
        if self.enable
        {
            self.modulo += ticks;

            let increment = (self.modulo / self.prescaler) as u16;
            let (value, overflow) = self.counter.overflowing_add(increment);

            // Reload initial value on overflow
            self.counter = if overflow {self.reload} else {value};
            self.modulo %= self.prescaler;

            overflow
        }
        else
        {
            false
        }
    }
}