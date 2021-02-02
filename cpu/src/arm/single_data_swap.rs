use crate::Bus;
use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, bus: &mut impl Bus, instr: u32) {
    execute(cpu, bus, decode(instr));
}

#[inline]
pub fn decode(instruction: u32) -> (bool, u32, u32, u32) {
    let b = instruction.bit(22);
    let rn = instruction.bits(19, 16);
    let rd = instruction.bits(15, 12);
    let rm = instruction.bits(3, 0);

    (b, rn, rd, rm)
}

#[inline]
pub fn execute(cpu: &mut CPU, memory: &mut impl Bus, (b, rn, rd, rm): (bool, u32, u32, u32)) {
    if b {
        let address = cpu.r[rn as usize];
        let temp = memory.load8(address);
        memory.store8(address, cpu.r[rm as usize] as u8);
        cpu.r[rd as usize] = temp as u32;

    // One internal cycle plus one load and one store
    // cpu.cycles += 1 + 2 * Bus::access_timing(address, 0);
    } else {
        let address = cpu.r[rn as usize];
        let temp = CPU::ldr(address, memory);
        memory.store32(address, cpu.r[rm as usize]);
        cpu.r[rd as usize] = temp;

        // cpu.cycles += 1 + 2 * Bus::access_timing(address, 2);
    }
}
