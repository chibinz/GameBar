pub mod disassemble;
pub mod instruction;
pub mod move_shifted;
pub mod add_subtract;
pub mod move_compare;

use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

/// Execute instruction
pub fn execute(cpu: &mut CPU, memory: &mut Memory) -> u32
{
    cpu.register.r[15] += 2;

    dispatch(cpu, memory, cpu.ir);

    0
}

pub fn fetch(cpu: &mut CPU, memory: &mut Memory)
{
    if cpu.flushed
    {
        cpu.ir = memory.load32(cpu.register.r[15]);
        cpu.register.r[15] += 2;
        cpu.flushed = false;
    }
    else
    {
        cpu.ir = memory.load32(cpu.register.r[15] - 2);
    }
}

pub fn dispatch(cpu: &mut CPU, memory: &mut Memory, instruction: u32)
{

}