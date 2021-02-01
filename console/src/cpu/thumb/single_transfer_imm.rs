use crate::cpu::CPU;
use crate::memory::Memory;
use crate::util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, memory: &mut Memory, instruction: u16) {
    execute(cpu, memory, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (u32, u32, u32, u32) {
    // Single and halfword data transfer use similar encoding format.
    // Thus is handled together.
    let bl = instruction.bits(12, 11);
    let offset5 = instruction.bits(10, 6);
    let rb = instruction.bits(5, 3);
    let rd = instruction.bits(2, 0);

    (bl, offset5, rb, rd)
}

#[inline]
fn execute(cpu: &mut CPU, memory: &mut Memory, (bl, offset5, rb, rd): (u32, u32, u32, u32)) {
    let base = cpu.r[rb as usize];
    let address = base + (offset5 << if bl.bit(1) { 0 } else { 2 });

    match bl {
        0b00 => memory.store32(address, cpu.r[rd as usize]),
        0b01 => cpu.r[rd as usize] = CPU::ldr(address, memory),
        0b10 => memory.store8(address, cpu.r[rd as usize] as u8),
        0b11 => cpu.r[rd as usize] = memory.load8(address) as u32,
        _ => unreachable!(),
    }

    // 1I + 1N
    cpu.cycles += 1 + Memory::access_timing(base, if bl.bit(1) { 0 } else { 2 });
}
