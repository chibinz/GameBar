use crate::shifter::shift_register;
use crate::Bus;
use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, bus: &mut impl Bus, instr: u32) {
    execute(cpu, bus, decode(instr));
}

#[inline]
pub fn decode(instr: u32) -> (bool, bool, bool, bool, u32, u32, u32, u32) {
    debug_assert_eq!(instr.bits(27, 26), 0b01);

    let i = instr.bit(25);
    let p = instr.bit(24);
    let u = instr.bit(23);
    let w = instr.bit(21);
    let rn = instr.bits(19, 16);
    let rd = instr.bits(15, 12);
    let offset = instr.bits(11, 0);

    // Instruction is later on dispatched using the l, b bits
    let lb = (instr >> 22 & 1) + ((instr >> 20 & 1) << 1);

    (i, p, u, w, lb, rn, rd, offset)
}

#[inline]
pub fn execute<T: ?Sized + Bus>(
    cpu: &mut CPU,
    bus: &mut T,
    (i, p, u, w, lb, rn, rd, offset): (bool, bool, bool, bool, u32, u32, u32, u32),
) {
    // 0 for i means immediate
    let noffset = if !i {
        offset
    } else {
        // Shifts does not set CPSR C flag
        shift_register(cpu, offset).0
    };

    let post = cpu.r(rn);
    let pre = if u {
        cpu.r(rn).wrapping_add(noffset)
    } else {
        cpu.r(rn).wrapping_sub(noffset)
    };

    let address = if p { pre } else { post };

    // When R15 is the source register, the stored value will be
    // address of the instruction plus 12
    let value = cpu.r(rd) + if rd == 15 { 4 } else { 0 };

    // Privileged write back bit not handled
    if w || !p {
        cpu.set_r(rn, pre);
    }

    // Misaligned word access handled in `memory.rs`
    match lb {
        0b00 => CPU::str(address, value, bus),
        0b01 => CPU::strb(address, value, bus),
        0b10 => cpu.set_r(rd, CPU::ldr(address, bus)),
        0b11 => cpu.set_r(rd, CPU::ldrb(address, bus)),
        _ => unreachable!(),
    }

    // cpu.cycles += 1 + Bus::access_timing(address, if lb.bit(0) { 0 } else { 2 });
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn byte_transfer() {
        let mut cpu = CPU::new();
        let mut bus = [0u8; 1024];
        let bus = bus.as_mut();

        bus.store8(0x01, 0xff);
        cpu.set_r(0, 0x00);

        // Immediate offset, pre-indexing, up offset, write back, load byte
        execute(&mut cpu, bus, (false, true, true, true, 0b11, 0, 1, 1));
        assert_eq!(cpu.r(1), 0xff);
        assert_eq!(cpu.r(0), 0x01);

        // Immediate offset, pre-indexing, down offset, no write back, store byte
        execute(&mut cpu, bus, (false, true, false, false, 0b01, 0, 1, 1));
        assert_eq!(CPU::ldrb(0x00, bus), 0xff);
    }

    #[test]
    fn word_transfer() {
        let mut cpu = CPU::new();
        let mut bus = [0u8; 1024];
        let bus = bus.as_mut();

        bus.store32(0x00, 0xdeadbeef);
        cpu.set_r(0, 0x00);

        // Immediate offset, post-indexing, up offset, write back, load word
        execute(&mut cpu, bus, (false, false, true, true, 0b10, 0, 1, 4));
        assert_eq!(cpu.r(1), 0xdeadbeef);
        assert_eq!(cpu.r(0), 0x04);

        cpu.set_r(1, 0);
        // Immediate offset, pre-indexing, down offset, write back, store word
        execute(&mut cpu, bus, (false, true, false, true, 0b00, 0, 1, 4));
        assert_eq!(CPU::ldr(0x00, bus), 0);
        assert_eq!(cpu.r(0), 0x00);
    }
}
