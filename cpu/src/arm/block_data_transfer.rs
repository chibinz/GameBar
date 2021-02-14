use crate::register::PSRMode::User;
use crate::Bus;
use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, bus: &mut impl Bus, instr: u32) {
    execute(cpu, bus, decode(instr));
}

#[inline]
pub fn decode(instr: u32) -> (bool, bool, bool, bool, bool, u32, u32) {
    debug_assert_eq!(instr.bits(27, 25), 0b100);

    let p = instr.bit(24);
    let u = instr.bit(23);
    let s = instr.bit(22);
    let w = instr.bit(21);
    let l = instr.bit(20);
    let rn = instr.bits(19, 16);
    let rlist = instr.bits(15, 0);

    (p, u, s, w, l, rn, rlist)
}

#[inline]
pub fn execute<T: ?Sized + Bus>(
    cpu: &mut CPU,
    bus: &mut T,
    (p, u, s, w, l, rn, rlist): (bool, bool, bool, bool, bool, u32, u32),
) {
    // Empty rlist not handled
    assert_ne!(rlist, 0);

    // Misaligned address not handled
    let mut address = cpu.r(rn);
    let original = address;

    let saved_cpsr = cpu.get_cpsr();

    if s {
        if !(l && rlist.bit(15)) {
            // Switch to User mode register bank
            cpu.set_cpsr(User as u32, false);
        }
    }

    // Whether or not the p bit is set, the final address after transfer
    // should be the same.
    if w {
        if u {
            cpu.set_r(rn, address + 4 * rlist.count_ones());
        } else {
            cpu.set_r(rn, address - 4 * rlist.count_ones());
        }
    }

    if p {
        address = if u { address + 4 } else { address - 4 }
    }

    // Empty list not handled
    for i in 0..16 {
        let j = if u { i } else { 15 - i };
        if rlist.bit(j) {
            if l {
                cpu.set_r(j, bus.load32(address));
            } else {
                bus.store32(address, cpu.r(j));

                if j == 15 {
                    bus.store32(address, cpu.r(15) + 4);
                }

                // The first register to be stored will store the
                // unchanged value.
                if w && j == rn && rlist.trailing_zeros() == rn {
                    bus.store32(address, original);
                }
            }

            address = if u { address + 4 } else { address - 4 };
        }
    }

    if s {
        // If the instruction is a LDM then SPSR_<mode> is transferred
        // to CPSR at the same time as R15 is loaded.
        if l && rlist.bit(15) {
            cpu.restore_cpsr();
        } else {
            cpu.set_cpsr(saved_cpsr, false);
        }
    }

    // One internal cycle plus n * memory waitstate
    // cpu.cycles += 1 + count_cycles(rlist) * Bus::access_timing(address, 2);
}

#[allow(dead_code)]
fn count_cycles(rlist: u32) -> i32 {
    rlist.count_ones() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_increment() {
        let mut cpu = CPU::new();
        let mut bus =  [0u8; 1024];
        let bus = bus.as_mut();

        cpu.set_spsr(cpu.get_cpsr(), false);

        for i in 0..16 {
            bus.as_mut().store32(0x00 + i * 4, i);
        }
        cpu.set_r(0, 0x00);

        // Write back bit is redundant because R0 is overwritten
        execute(
            &mut cpu,
             bus,
            (false, true, true, true, true, 0, 0xffff),
        );
        for i in 0..15 {
            assert_eq!(cpu.r(i), i);
        }

        // PC should be word aligned
        assert_eq!(cpu.r(15), 16);
    }

    #[test]
    fn pre_increment() {
        let mut cpu = CPU::new();
        let mut bus = [0u8; 1024];
        let bus = bus.as_mut();

        bus.store32(0x04, 1);
        bus.store32(0x08, 2);
        bus.store32(0x0c, 3);
        cpu.set_r(0, 0x00);

        execute(
            &mut cpu,
            bus,
            (true, true, true, true, true, 0, 0x000e),
        );
        assert_eq!(cpu.r(1), 1);
        assert_eq!(cpu.r(2), 2);
        assert_eq!(cpu.r(3), 3);
        assert_eq!(cpu.r(0), 0x0c);
    }

    #[test]
    fn post_decrement() {
        let mut cpu = CPU::new();
        let mut bus = [0u8; 1024];
        let bus = bus.as_mut();

        bus.store32(0x04, 1);
        bus.store32(0x08, 2);
        bus.store32(0x0c, 3);
        cpu.set_r(0, 0x0c);

        execute(
            &mut cpu,
           bus,
            (false, false, true, true, true, 0, 0x000e),
        );
        assert_eq!(cpu.r(1), 1);
        assert_eq!(cpu.r(2), 2);
        assert_eq!(cpu.r(3), 3);
        assert_eq!(cpu.r(0), 0x00);
    }

    #[test]
    fn pre_decrement() {
        let mut cpu = CPU::new();
        let mut bus = [0u8; 1024];
        let bus = bus.as_mut();

        bus.store32(0x00, 1);
        bus.store32(0x04, 2);
        bus.store32(0x08, 3);
        cpu.set_r(0, 0x0c);

        // Subtraction overflow
        // execute(
        //     &mut cpu,
        //     &mut bus,
        //     (true, false, true, true, true, 0, 0x1110),
        // );
        // assert_eq!(cpu.r(4), 1);
        // assert_eq!(cpu.r(8), 2);
        // assert_eq!(cpu.r(12), 3);
        // assert_eq!(cpu.r(0), 0x00);
    }
}
