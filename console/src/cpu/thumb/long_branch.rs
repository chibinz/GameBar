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
    let h = instruction.bit(11);
    let offset = instruction.bits(10, 0);

    (h, offset)
}

#[inline]
fn execute(cpu: &mut CPU, (h, offset): (bool, u32))
{
    if h
    {
        let temp = cpu.r[15] - 2;
        cpu.r[15] = cpu.r[14].wrapping_add(offset << 1);
        cpu.r[14] = temp | 1;

        cpu.flush();
    }
    else
    {
        cpu.r[14] = cpu.r[15].wrapping_add((sign_extend(offset, 10) as u32) << 12);
    }
}