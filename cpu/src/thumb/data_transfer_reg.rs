use crate::Bus;
use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, memory: &mut impl Bus, instr: u16) {
    execute(cpu, memory, decode(instr));
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
        0b000 => {
            bus.store32(address, cpu.r[rd as usize]);
            2
        }
        0b001 => {
            bus.store16(address, cpu.r[rd as usize] as u16);
            1
        }
        0b010 => {
            bus.store8(address, cpu.r[rd as usize] as u8);
            0
        }
        0b011 => {
            cpu.r[rd as usize] = bus.load8(address) as i8 as i32 as u32;
            0
        }
        0b100 => {
            cpu.r[rd as usize] = CPU::ldr(address, bus);
            2
        }
        0b101 => {
            cpu.r[rd as usize] = CPU::ldrh(address, bus);
            1
        }
        0b110 => {
            cpu.r[rd as usize] = bus.load8(address) as u32;
            0
        }
        0b111 => {
            cpu.r[rd as usize] = CPU::ldrsh(address, bus);
            1
        }
        _ => unreachable!(),
    };

    // cpu.cycles += 1 + Bus::access_timing(address, size);
}
