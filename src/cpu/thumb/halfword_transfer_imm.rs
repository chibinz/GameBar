use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

#[inline]
pub fn interpret(cpu: &mut CPU, memory: &mut Memory, instruction: u16)
{
    execute(cpu, memory, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (bool, u32, u32, u32)
{
    // Single and halfword data transfer use similar encoding format.
    // Thus is handled together.
    let l = instruction.bit(11);
    let offset5 = instruction.bits(10, 6);
    let rb = instruction.bits(5, 3);
    let rd = instruction.bits(2, 0);

    (l, offset5, rb, rd)
}

#[inline]
fn execute(cpu: &mut CPU, memory: &mut Memory, (l, offset5, rb, rd): (bool, u32, u32, u32))
{
    let address = cpu.r[rb as usize] + (offset5 << 1);

    if l
    {
        cpu.r[rd as usize] = memory.load16(address) as u32;
    }
    else
    {
        memory.store16(address, cpu.r[rd as usize] as u16);
    }
}