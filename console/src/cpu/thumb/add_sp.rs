use crate::util::*;
use crate::cpu::CPU;

#[inline]
pub fn interpret(cpu: &mut CPU, instruction: u16)
{
    execute(cpu, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (bool, u32)
{
    let s = instruction.bit(7);
    let sword7 = instruction.bits(6, 0);

    (s, sword7)
}

#[inline]
fn execute(cpu: &mut CPU, (s, sword7): (bool, u32))
{
    if s
    {
        cpu.r[13] -= sword7 << 2;
    }
    else
    {
        cpu.r[13] += sword7 << 2;
    }
}