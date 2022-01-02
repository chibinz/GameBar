use crate::register::PsrMode::User;
use crate::Bus;
use crate::Cpu;
use util::*;

#[inline]
pub fn interpret(cpu: &mut Cpu, bus: &mut impl Bus, instr: u32) {
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
pub fn execute(
    cpu: &mut Cpu,
    bus: &mut impl Bus,
    (p, u, s, w, l, rn, rlist): (bool, bool, bool, bool, bool, u32, u32),
) {
    // LDM and STM are forced align
    // Empty rlist not handled

    let mut addr = cpu.r(rn);
    let saved_cpsr = cpu.get_cpsr();

    if s && !(l && rlist.bit(15)) {
        // Switch to User mode register bank
        cpu.set_cpsr(User as u32, false);
    }

    // Whether or not the p bit is set, the final address after transfer
    // should be the same.
    let step = if u { 4 } else { 4u32.wrapping_neg() };
    let (rlist, num_regs) = if rlist == 0 {
        (0x8000, 16)
    } else {
        (rlist, rlist.count_ones())
    };
    let final_addr = addr.wrapping_add(step.wrapping_mul(num_regs));
    let indices = rlist_to_index(rlist);
    let mut regs = indices.iter();

    // Fucking doesn't make sense...
    addr = match (u, p) {
        (true, true) => addr.wrapping_add(4),
        (true, false) => addr,
        (false, true) => final_addr,
        (false, false) => final_addr.wrapping_add(4),
    };

    let &first = regs.next().unwrap();
    if l {
        cpu.set_r(first, Cpu::ldr(addr & !0b11, bus));
    } else {
        Cpu::str(addr & !0b11, cpu.r(first) + if first == 15 {4} else {0}, bus);
    };
    addr = addr.wrapping_add(4);

    // Write back after second cycle
    if w && !(l && rlist.bit(rn)) { cpu.set_r(rn, final_addr); }

    for &r in regs {
        if l {
            cpu.set_r(r, Cpu::ldr(addr & !0b11, bus));
        } else {
            Cpu::str(addr & !0b11, cpu.r(r) + if r == 15 {4} else {0}, bus);
        };

        addr = addr.wrapping_add(4);
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

fn rlist_to_index(mut rlist: u32) -> Vec<u32> {
    let mut ret = Vec::with_capacity(16);

    while rlist > 0 {
        let i = rlist.trailing_zeros();
        rlist &= !(1 << i);
        ret.push(i);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::DummyBus;

    #[test]
    fn post_increment() {
        let mut cpu = Cpu::new();
        let mut bus = DummyBus::new();

        cpu.set_spsr(cpu.get_cpsr(), false);

        for i in 0..16 {
            bus.store32( i * 4, i as u32);
        }
        cpu.set_r(0, 0x00);

        // Write back bit is redundant because R0 is overwritten
        execute(
            &mut cpu,
            &mut bus,
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
        let mut cpu = Cpu::new();
        let mut bus = DummyBus::new();

        bus.store32(0x04, 1);
        bus.store32(0x08, 2);
        bus.store32(0x0c, 3);
        cpu.set_r(0, 0x00);

        execute(
            &mut cpu,
            &mut bus,
            (true, true, true, true, true, 0, 0x000e),
        );
        assert_eq!(cpu.r(1), 1);
        assert_eq!(cpu.r(2), 2);
        assert_eq!(cpu.r(3), 3);
        assert_eq!(cpu.r(0), 0x0c);
    }

    #[test]
    fn post_decrement() {
        let mut cpu = Cpu::new();
        let mut bus = DummyBus::new();

        bus.store32(0x04, 1);
        bus.store32(0x08, 2);
        bus.store32(0x0c, 3);
        cpu.set_r(0, 0x0c);

        execute(
            &mut cpu,
            &mut bus,
            (false, false, true, true, true, 0, 0x000e),
        );
        assert_eq!(cpu.r(1), 1);
        assert_eq!(cpu.r(2), 2);
        assert_eq!(cpu.r(3), 3);
        assert_eq!(cpu.r(0), 0x00);
    }

    #[test]
    fn pre_decrement() {
        let mut cpu = Cpu::new();
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
