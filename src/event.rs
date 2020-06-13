//! Prototypical event scheduler
use std::collections::VecDeque;

enum Event
{
    HDraw,
    HBlank,
    VBlank,
    CPU,
    DMA,
    Timer,
}

struct Scheduler
{
    pub tick_til_next_event: i32,
    pub total_tick: i64,
    pub events: VecDeque<Event>,
}