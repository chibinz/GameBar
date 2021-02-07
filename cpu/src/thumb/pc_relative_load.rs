use crate::Bus;
use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, bus: &mut impl Bus, instr: u16) {
    execute(cpu, bus, decode(instr));
}

#[inline]
fn decode(instr: u16) -> (u32, u32) {
    let rd = instr.bits(10, 8);
    let word8 = instr.bits(7, 0);

    (rd, word8)
}

#[inline]
fn execute(cpu: &mut CPU, bus: &mut impl Bus, (rd, word8): (u32, u32)) {
    // Bit 1 of PC is forced to 0 to ensure it is word aligned.
    let address = (cpu.r(15) & 0xfffffffc) + (word8 << 2);

    cpu.set_r(rd, CPU::ldr(address, bus));

    // cpu.cycles += 1 + Bus::access_timing(address, 2);
}
