use crate::Bus;
use crate::Cpu;
use util::*;

#[inline]
pub fn interpret(cpu: &mut Cpu, bus: &mut impl Bus, instr: u16) {
    execute(cpu, bus, decode(instr));
}

#[inline]
fn decode(instr: u16) -> (u32, u32, u32, u32) {
    // Single and halfword data transfer use similar encoding format.
    // Thus is handled together.
    let lbh = instr.bits(11, 9);
    let ro = instr.bits(8, 6);
    let rb = instr.bits(5, 3);
    let rd = instr.bits(2, 0);

    (lbh, ro, rb, rd)
}

#[inline]
fn execute(cpu: &mut Cpu, bus: &mut impl Bus, (lbh, ro, rb, rd): (u32, u32, u32, u32)) {
    let address = cpu.r(rb).wrapping_add(cpu.r(ro));

    // Misaligned halfword access is not handled
    match lbh {
        0b000 => Cpu::str(address, cpu.r(rd), bus),
        0b001 => Cpu::strh(address, cpu.r(rd), bus),
        0b010 => Cpu::strb(address, cpu.r(rd), bus),
        0b011 => cpu.set_r(rd, Cpu::ldrsb(address, bus)),
        0b100 => cpu.set_r(rd, Cpu::ldr(address, bus)),
        0b101 => cpu.set_r(rd, Cpu::ldrh(address, bus)),
        0b110 => cpu.set_r(rd, Cpu::ldrb(address, bus)),
        0b111 => cpu.set_r(rd, Cpu::ldrsh(address, bus)),
        _ => unreachable!(),
    };

    // cpu.cycles += 1 + Bus::access_timing(address, size);
}
