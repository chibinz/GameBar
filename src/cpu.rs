pub mod register;
pub mod arm;
pub mod thumb;

use register::Register;

#[derive(Debug)]
pub enum Mode
{
    ARM,
    THUMB,
}

pub struct CPU
{
    pub register: Register,
    pub mode: Mode,
}

impl CPU
{
    pub fn new() -> Self
    {
        Self
        {
            register: Register::new(),
            mode: Mode::ARM,
        }
    }

}