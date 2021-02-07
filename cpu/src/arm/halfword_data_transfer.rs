use crate::Bus;
use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, bus: &mut impl Bus, instr: u32) {
    execute(cpu, bus, decode(instr));
}

#[inline]
pub fn decode(instr: u32) -> (bool, bool, bool, bool, u32, u32, u32, u32) {
    debug_assert_eq!(instr.bits(27, 25), 0b000);
    debug_assert_eq!(instr.bit(7), true);
    debug_assert_eq!(instr.bit(4), true);

    let p = instr.bit(24);
    let u = instr.bit(23);
    // Offset can be a register or an immediate depending on bit 22
    let i = instr.bit(22);
    let w = instr.bit(21);
    let rn = instr.bits(19, 16);
    let rd = instr.bits(15, 12);

    // Instruction is later on dispatched by the l, s, h bits
    // Note that the shift operator takes precedence over add operator
    let lsh = ((instr >> 20 & 1) << 2) + instr.bits(6, 5);

    let offset = (instr.bits(11, 8) << 4) + instr.bits(3, 0);

    (p, u, i, w, lsh, rn, rd, offset)
}

#[inline]
pub fn execute(
    cpu: &mut CPU,
    bus: &mut impl Bus,
    (p, u, i, w, lsh, rn, rd, offset): (bool, bool, bool, bool, u32, u32, u32, u32),
) {
    let noffset = if i { offset } else { cpu.r[offset as usize] };

    let post = cpu.r[rn as usize];
    let pre = if u {
        cpu.r[rn as usize] + noffset
    } else {
        cpu.r[rn as usize] - noffset
    };

    // When R15 is the source register, the stored value will be
    // address of the instruction plus 12
    let address = if p { pre } else { post };
    let adjust = if rd == 15 {4} else {0};
    let value = cpu.r[rd as usize] + adjust;

    // Writeback may be overwritten if rn = rd
    if w || !p {
        cpu.r[rn as usize] = pre;
    }

    match lsh {
        0b001 => CPU::strh(address, value, bus),
        0b010 => CPU::strb(address, value, bus),
        0b011 => CPU::strh(address, value, bus),
        0b101 => cpu.r[rd as usize] = CPU::ldrh(address, bus) + adjust,
        0b110 => cpu.r[rd as usize] = CPU::ldrsb(address, bus) + adjust,
        0b111 => cpu.r[rd as usize] = CPU::ldrsh(address, bus) + adjust,
        _ => unreachable!(),
    }

    // One internal cycle plus memory waitstate
    // cpu.cycles += 1 + Bus::access_timing(address, 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::DummyBus;

    #[test]
    fn decode_strh() {
        let instruction = 0b0000_000_11111_1010_0101_1111_1011_1111;
        assert_eq!(
            decode(instruction),
            (true, true, true, true, 0b101, 0b1010, 0b0101, 0b11111111)
        );
    }

    #[test]
    fn halfword_transfer() {
        let mut cpu = CPU::new();
        let mut bus = DummyBus::new();

        bus.store16(0x02000000, 0xdead);
        cpu.r[0] = 0x02000000;

        // Post-indexing, up offset, write back, load unsigned halfword
        // base = r0, dst = r1, offset = 2
        execute(
            &mut cpu,
            &mut bus,
            (false, true, true, true, 0b101, 0, 1, 2),
        );
        assert_eq!(cpu.r[1], 0xdead);
        assert_eq!(cpu.r[0], 0x02000002);

        bus.store8(0x02000001, 0xff);
        // Pre-indexing, down offset, write back, load signed byte
        // base = r0, src = r1, offset = 1
        execute(
            &mut cpu,
            &mut bus,
            (true, false, true, true, 0b110, 0, 1, 1),
        );
        assert_eq!(cpu.r[1], 0xffffffff);
        assert_eq!(cpu.r[0], 0x02000001);
    }
}
