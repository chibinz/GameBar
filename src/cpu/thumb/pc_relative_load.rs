use crate::util::*;
use crate::cpu::CPU;

#[inline]
pub fn decode_execute(cpu: &mut CPU, instruction: u16)
{
    execute(cpu, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (u32, u32)
{
    let rd = instruction.bits(10, 8);
    let word8 = instruction.bits(7, 0);

    (rd, word8)
}
 
#[inline]
fn execute(cpu: &mut CPU, (rd, word8): (u32, u32))
{   
    // Bit 1 of PC is forced to 0 to ensure it is word aligned.
    cpu.r[rd as usize] = (cpu.r[15] & 0xfffffffc) + (word8 << 2);
}