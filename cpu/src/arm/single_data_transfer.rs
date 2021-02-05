use crate::barrel_shifter::shift_register;
use crate::register::PSRBit::C;
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
pub fn execute(
    cpu: &mut CPU,
    bus: &mut impl Bus,
    (i, p, u, w, lb, rn, rd, offset): (bool, bool, bool, bool, u32, u32, u32, u32),
) {
    // Shifts does not set CPSR C flag
    let carry = cpu.get_cpsr_bit(C);

    // 0 for i means immediate
    let noffset = if !i {
        offset
    } else {
        0// shift_register(cpu, offset)
    };
    cpu.set_cpsr_bit(C, carry);

    let post = cpu.r[rn as usize];
    let pre = if u {
        cpu.r[rn as usize].wrapping_add(noffset)
    } else {
        cpu.r[rn as usize].wrapping_sub(noffset)
    };

    let address = if p { pre } else { post };

    // When R15 is the source register, the stored value will be
    // address of the instruction plus 12
    let value = cpu.r[rd as usize] + if rd == 15 { 4 } else { 0 };

    // Privileged write back bit not handled
    if w || !p {
        cpu.r[rn as usize] = pre;
    }

    // Misaligned word access handled in `memory.rs`
    match lb {
        0b00 => bus.store32(address, value),
        0b01 => bus.store8(address, value as u8),
        0b10 => {
            cpu.r[rd as usize] = CPU::ldr(address, bus);
            if rd == 15 {
                cpu.flush()
            }
        }
        0b11 => {
            cpu.r[rd as usize] = bus.load8(address) as u32;
            if rd == 15 {
                cpu.flush()
            }
        }
        _ => unreachable!(),
    }

    // cpu.cycles += 1 + Bus::access_timing(address, if lb.bit(0) { 0 } else { 2 });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::DummyBus;

    #[test]
    fn byte_transfer() {
        let mut cpu = CPU::new();
        let mut bus = DummyBus::new();

        bus.store8(0x02000001, 0xff);
        cpu.r[0] = 0x02000000;

        // Immediate offset, pre-indexing, up offset, write back, load byte
        execute(&mut cpu, &mut bus, (false, true, true, true, 0b11, 0, 1, 1));
        assert_eq!(cpu.r[1], 0xff);
        assert_eq!(cpu.r[0], 0x02000001);

        // Immediate offset, pre-indexing, down offset, no write back, store byte
        execute(
            &mut cpu,
            &mut bus,
            (false, true, false, false, 0b01, 0, 1, 1),
        );
        assert_eq!(bus.load8(0x02000000), 0xff);
    }

    #[test]
    fn word_transfer() {
        let mut cpu = CPU::new();
        let mut bus = DummyBus::new();

        bus.store32(0x02000000, 0xdeadbeef);
        cpu.r[0] = 0x02000000;

        // Immediate offset, post-indexing, up offset, write back, load word
        execute(
            &mut cpu,
            &mut bus,
            (false, false, true, true, 0b10, 0, 1, 4),
        );
        assert_eq!(cpu.r[1], 0xdeadbeef);
        assert_eq!(cpu.r[0], 0x02000004);

        cpu.r[1] = 0;
        // Immediate offset, pre-indexing, down offset, write back, store word
        execute(
            &mut cpu,
            &mut bus,
            (false, true, false, true, 0b00, 0, 1, 4),
        );
        assert_eq!(bus.load32(0x02000000), 0);
        assert_eq!(cpu.r[0], 0x02000000);
    }
}
