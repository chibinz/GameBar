pub mod register;
pub mod arm;
pub mod thumb;

use register::Register;

// #[derive(Debug)]
// pub enum Mode
// {
//     ARM,
//     THUMB,
// }

pub struct CPU
{
    pub register: Register,
}

impl CPU
{
    pub fn new() -> Self
    {
        Self
        {
            register: Register::new(),
        }
    }
}