use crate::cpu::CPU;
use crate::memory::Memory;
use crate::util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, memory: &mut Memory, instruction: u16) {
    execute(cpu, memory, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (bool, u32, u32) {
    let l = instruction.bit(11);
    let rd = instruction.bits(10, 8);
    let word8 = instruction.bits(7, 0);

    (l, rd, word8)
}

#[inline]
fn execute(cpu: &mut CPU, memory: &mut Memory, (l, rd, word8): (bool, u32, u32)) {
    let address = cpu.r[13] + (word8 << 2);

    if l {
        cpu.r[rd as usize] = CPU::ldr(address, memory);
    } else {
        memory.store32(address, cpu.r[rd as usize]);
    }

    cpu.cycles += 1 + Memory::access_timing(address, 2);
}
