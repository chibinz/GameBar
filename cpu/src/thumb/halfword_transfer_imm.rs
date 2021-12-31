use crate::Bus;
use crate::Cpu;
use util::*;

#[inline]
pub fn interpret(cpu: &mut Cpu, bus: &mut impl Bus, instr: u16) {
    execute(cpu, bus, decode(instr));
}

#[inline]
fn decode(instr: u16) -> (bool, u32, u32, u32) {
    // Single and halfword data transfer use similar encoding format.
    // Thus is handled together.
    let l = instr.bit(11);
    let offset5 = instr.bits(10, 6);
    let rb = instr.bits(5, 3);
    let rd = instr.bits(2, 0);

    (l, offset5, rb, rd)
}

#[inline]
fn execute(cpu: &mut Cpu, bus: &mut impl Bus, (l, offset5, rb, rd): (bool, u32, u32, u32)) {
    let address = cpu.r(rb) + (offset5 << 1);

    if l {
        cpu.set_r(rd, Cpu::ldrh(address, bus));
    } else {
        Cpu::strh(address, cpu.r(rd), bus);
    }

    // cpu.cycles += 1 + Bus::access_timing(address, 1);
}
