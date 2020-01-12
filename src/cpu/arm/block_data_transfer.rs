use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

pub fn decode_execute(cpu: &mut CPU, memory: &mut Memory, instruction: u32)
{
    execute(cpu, memory, decode(instruction));
}

#[inline]
pub fn decode(instruction: u32) -> (bool, bool, bool, bool, bool, u32, u32)
{
    debug_assert_eq!(instruction.bits(27, 25), 0b100);

    let p     = instruction.bit(24);
    let u     = instruction.bit(23);
    let s     = instruction.bit(22);
    let w     = instruction.bit(21);
    let l     = instruction.bit(20);
    let rn    = instruction.bits(19, 16);
    let rlist = instruction.bits(15, 0);

    (p, u, s, w, l, rn, rlist)
}

#[inline]
pub fn execute(cpu: &mut CPU, memory: &mut Memory,
    (p, u, s, w, l, rn, rlist): (bool, bool, bool, bool, bool, u32, u32))
{
    let mut address = cpu.r[rn as usize];

    if p
    {
        address = if u {address + 4} else {address - 4}
    }

    for i in 0..16
    {
        let j = if u {i} else {15 - i};
        if bit(rlist, j)
        {
            if l
            {
                cpu.r[j as usize] = memory.load32(address);
            }
            else
            {
                memory.store32(address, cpu.r[j as usize]);
            }

            address = if u {address + 4} else {address - 4};
        }
    }

    if s && bit(rlist, 0)
    {
        // If the instruction is a LDM then SPSR_<mode> is transferred
        // to CPSR at the same time as R15 is loaded.
        if l
        {
            cpu.restore_cpsr();
        }
    }

    // Whether or not the p bit is set, the final address after transfer 
    // should be the same. In the pre-increment case, the final address 
    // needs to be adjusted
    if w && !bit(rlist, rn)
    {
        if p
        {
            address = if u {address - 4} else {address + 4}
        }

        cpu.r[rn as usize] = address
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn post_increment()
    {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        for i in 0..16
        {
            memory.store32(0x02000000 + i * 4, i);
        }
        cpu.r[0] = 0x02000000;

        // Write back bit is redundant because R0 is overwritten
        execute(&mut cpu, &mut memory, (false, true, true, true, true, 0, 0xffff));
        for i in 0..16
        {
            assert_eq!(cpu.r[i as usize], i);
        }
    }

    #[test]
    fn pre_increment()
    {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        memory.store32(0x02000004, 1);
        memory.store32(0x02000008, 2);
        memory.store32(0x0200000c, 3);
        cpu.r[0] = 0x02000000;

        execute(&mut cpu, &mut memory, (true, true, true, true, true, 0, 0x000e));
        assert_eq!(cpu.r[1], 1);
        assert_eq!(cpu.r[2], 2);
        assert_eq!(cpu.r[3], 3);
        assert_eq!(cpu.r[0], 0x0200000c);
    }

    #[test]
    fn post_decrement()
    {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        memory.store32(0x02000004, 1);
        memory.store32(0x02000008, 2);
        memory.store32(0x0200000c, 3);
        cpu.r[0] = 0x0200000c;

        execute(&mut cpu, &mut memory, (false, false, true, true, true, 0, 0x000e));
        assert_eq!(cpu.r[1], 1);
        assert_eq!(cpu.r[2], 2);
        assert_eq!(cpu.r[3], 3);
        assert_eq!(cpu.r[0], 0x02000000);
    }

    #[test]
    fn pre_decrement()
    {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        memory.store32(0x02000000, 1);
        memory.store32(0x02000004, 2);
        memory.store32(0x02000008, 3);
        cpu.r[0] = 0x0200000c;

        execute(&mut cpu, &mut memory, (true, false, true, true, true, 0, 0x1110));
        assert_eq!(cpu.r[4], 1);
        assert_eq!(cpu.r[8], 2);
        assert_eq!(cpu.r[12], 3);
        assert_eq!(cpu.r[0], 0x02000000);
    }
}
