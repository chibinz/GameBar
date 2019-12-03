pub mod disassemble;
pub mod data_processing;
pub mod psr_transfer;
pub mod branch_long;
pub mod branch_exchange;
pub mod multiply_accumulate;
pub mod multiply_long_accumulate;
pub mod single_data_transfer;
pub mod halfword_data_transfer;

use crate::util::*;
use crate::cpu::CPU;

impl CPU
{
    /// Execute instruction
    pub fn execute(&mut self, instruction: u32) -> u32
    {
        let cond = bits(instruction, 31, 28);
        if self.check_condition(cond)
        {
            data_processing::execute(self, data_processing::decode(instruction));
        }
        
        self.register.r[15] += 4;

        return 0;
    }

}

#[cfg(test)]
mod tests
{
    use super::*;

}
