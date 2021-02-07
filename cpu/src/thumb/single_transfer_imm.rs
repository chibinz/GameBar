use crate::Bus;
use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, bus: &mut impl Bus, instr: u16) {
    execute(cpu, bus, decode(instr));
}

#[inline]
fn decode(instr: u16) -> (u32, u32, u32, u32) {
    // Single and halfword data transfer use similar encoding format.
    // Thus is handled together.
    let bl = instr.bits(12, 11);
    let offset5 = instr.bits(10, 6);
    let rb = instr.bits(5, 3);
    let rd = instr.bits(2, 0);

    (bl, offset5, rb, rd)
}

#[inline]
fn execute(cpu: &mut CPU, bus: &mut impl Bus, (bl, offset5, rb, rd): (u32, u32, u32, u32)) {
    let base = cpu.r(rb);
    let address = base + (offset5 << if bl.bit(1) { 0 } else { 2 });

    match bl {
        0b00 => CPU::str(address, cpu.r(rd), bus),
        0b01 => cpu.set_r(rd, CPU::ldr(address, bus)),
        0b10 => CPU::strb(address, cpu.r(rd), bus),
        0b11 => cpu.set_r(rd, CPU::ldrb(address, bus)),
        _ => unreachable!(),
    }

    // 1I + 1N
    // cpu.cycles += 1 + Bus::access_timing(base, if bl.bit(1) { 0 } else { 2 });
}
