use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

#[inline]
pub fn decode_execute(cpu: &mut CPU, memory: &mut Memory, instruction: u16)
{
    execute(cpu, memory, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (bool, u32, u32)
{
    let l = instruction.bit(11);
    let rd = instruction.bits(10, 8);
    let word8 = instruction.bits(7, 0);

    (l, rd, word8)
}
 
#[inline]
fn execute(cpu: &mut CPU, memory: &mut Memory, (l, rd, word8): (bool, u32, u32))
{   
    // Bit 1 of SP is forced to 0 to ensure it is word aligned.
    let address = (cpu.r[13] & 0xfffffffc) + (word8 << 2);

    if l
    {
        cpu.r[rd as usize] = memory.load32(address);
    }
    else
    {
        memory.store32(address, cpu.r[rd as usize]);
    }
}