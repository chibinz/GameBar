use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

#[inline]
pub fn interpret(cpu: &mut CPU, memory: &mut Memory, instruction: u16)
{
    execute(cpu, memory, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (u32, u32, u32, u32)
{
    // Single and halfword data transfer use similar encoding format.
    // Thus is handled together.
    let lbh = instruction.bits(11, 9);
    let ro = instruction.bits(8, 6);
    let rb = instruction.bits(5, 3);
    let rd = instruction.bits(2, 0);

    (lbh, ro, rb, rd)
}

#[inline]
fn execute(cpu: &mut CPU, memory: &mut Memory, (lbh, ro, rb, rd): (u32, u32, u32, u32))
{
    let address = cpu.r[rb as usize].wrapping_add(cpu.r[ro as usize]);

    // Misaligned halfword access is not handled
    let size = match lbh
    {
        0b000 => {memory.store32(address, cpu.r[rd as usize]); 2},
        0b001 => {memory.store16(address, cpu.r[rd as usize] as u16); 1},
        0b010 => {memory.store8(address, cpu.r[rd as usize] as u8); 0},
        0b011 => {cpu.r[rd as usize] = memory.load8(address) as i8 as i32 as u32; 0},
        0b100 => {cpu.r[rd as usize] = memory.load32(address); 2},
        0b101 => {cpu.r[rd as usize] = memory.load16(address) as u32; 1},
        0b110 => {cpu.r[rd as usize] = memory.load8(address) as u32; 0},
        0b111 => {cpu.r[rd as usize] = memory.load16(address) as i16 as i32 as u32; 1},
        _    => unreachable!(),
    };

    cpu.cycles += 1 + Memory::access_timing(address, size);
}