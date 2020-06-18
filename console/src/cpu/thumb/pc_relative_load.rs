use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

#[inline]
pub fn interpret(cpu: &mut CPU, memory: &mut Memory, instruction: u16)
{
    execute(cpu, memory, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (u32, u32)
{
    let rd = instruction.bits(10, 8);
    let word8 = instruction.bits(7, 0);

    (rd, word8)
}

#[inline]
fn execute(cpu: &mut CPU, memory: &mut Memory, (rd, word8): (u32, u32))
{
    // Bit 1 of PC is forced to 0 to ensure it is word aligned.
    let address = (cpu.r[15] & 0xfffffffc) + (word8 << 2);

    cpu.r[rd as usize] = memory.load32(address);

    cpu.cycles += 1 + Memory::cpu_access_timing(address, 2);
}