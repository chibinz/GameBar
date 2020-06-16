use crate::keypad::Keypad;

impl Keypad
{
    pub fn get_input(&self) -> u16
    {
        self.keyinput
    }

    pub fn get_control(&self) -> u16
    {
        self.keycnt
    }

    pub fn set_control(&mut self, value: u16)
    {
        self.keycnt = value;
    }
}