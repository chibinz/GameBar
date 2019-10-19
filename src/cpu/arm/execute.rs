use crate::cpu::CPU;

impl CPU
{
    /// Execute instruction
    pub fn execute(&mut self, instruction: u32) -> u32
    {
        let cond = instruction >> 28 & 0b1111;
        if !self.check_condition(cond)
        {
            return 0;
        }

        self.register.r[15] += 4;

        return 0;
    }


}

#[cfg(test)]
mod test
{
    use super::*;

}