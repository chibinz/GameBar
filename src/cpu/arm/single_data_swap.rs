use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

#[inline]
pub fn decode_execute(cpu: &mut CPU, memory: &mut Memory, instruction: u32)
{
    execute(cpu, memory, decode(instruction));
}

#[inline]
pub fn decode(instruction: u32) -> (bool, u32, u32, u32)
{
    let b = instruction.bit(22);
    let rn = instruction.bits(19, 16);
    let rd = instruction.bits(15, 12);
    let rm = instruction.bits(3, 0);

    (b, rn, rd, rm)
}

#[inline]
pub fn execute(cpu: &mut CPU, memory: &mut Memory, (b, rn, rd, rm): (bool, u32, u32, u32))
{
    if b
    {
        let temp = memory.load8(cpu.r[rn as usize]);
        memory.store8(cpu.r[rn as usize], cpu.r[rm as usize] as u8);
        cpu.r[rd as usize] = temp as u32;
    }
    else
    {
        let temp = memory.load32(cpu.r[rn as usize]);
        memory.store32(cpu.r[rn as usize], cpu.r[rm as usize]);
        cpu.r[rd as usize] = temp;
    }
}