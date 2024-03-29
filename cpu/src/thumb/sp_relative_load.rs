use crate::Bus;
use crate::Cpu;
use util::*;

#[inline]
pub fn interpret(cpu: &mut Cpu, bus: &mut impl Bus, instr: u16) {
    execute(cpu, bus, decode(instr));
}

#[inline]
fn decode(instr: u16) -> (bool, u32, u32) {
    let l = instr.bit(11);
    let rd = instr.bits(10, 8);
    let word8 = instr.bits(7, 0);

    (l, rd, word8)
}

#[inline]
fn execute(cpu: &mut Cpu, bus: &mut impl Bus, (l, rd, word8): (bool, u32, u32)) {
    let address = cpu.r(13) + (word8 << 2);

    if l {
        cpu.set_r(rd, Cpu::ldr(address, bus));
    } else {
        Cpu::str(address, cpu.r(rd), bus);
    }

    // cpu.cycles += 1 + Bus::access_timing(address, 2);
}
