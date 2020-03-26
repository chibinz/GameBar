use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

use crate::cpu::arm::block_data_transfer;

#[inline]
pub fn interpret(cpu: &mut CPU, memory: &mut Memory, instruction: u16)
{
    execute(cpu, memory, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (bool, u32, u32)
{
    let l = instruction.bit(11);
    let rb = instruction.bits(10, 8);
    let rlist = instruction.bits(7, 0);

    (l, rb, rlist)
}

#[inline]
fn execute(cpu: &mut CPU, memory: &mut Memory, (l, rb, rlist): (bool, u32, u32))
{
    // P = 0, U = 1, S = 0, W = true, L = l
    block_data_transfer::execute(cpu, memory, (false, true, false, true, l, rb, rlist));
}