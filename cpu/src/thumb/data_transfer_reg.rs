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
    let lbh = instr.bits(11, 9);
    let ro = instr.bits(8, 6);
    let rb = instr.bits(5, 3);
    let rd = instr.bits(2, 0);

    (lbh, ro, rb, rd)
}

#[inline]
fn execute(cpu: &mut CPU, bus: &mut impl Bus, (lbh, ro, rb, rd): (u32, u32, u32, u32)) {
    let address = cpu.r[rb as usize].wrapping_add(cpu.r[ro as usize]);

    // Misaligned halfword access is not handled
    match lbh {
        0b000 => CPU::str(address, cpu.r[rd as usize], bus),
        0b001 => CPU::strh(address, cpu.r[rd as usize], bus),
        0b010 => CPU::strb(address, cpu.r[rd as usize], bus),
        0b011 => cpu.r[rd as usize] = CPU::ldrsb(address, bus),
        0b100 => cpu.r[rd as usize] = CPU::ldr(address, bus),
        0b101 => cpu.r[rd as usize] = CPU::ldrh(address, bus),
        0b110 => cpu.r[rd as usize] = CPU::ldrb(address, bus),
        0b111 => cpu.r[rd as usize] = CPU::ldrsh(address, bus),
        _ => unreachable!(),
    };

    // cpu.cycles += 1 + Bus::access_timing(address, size);
}
