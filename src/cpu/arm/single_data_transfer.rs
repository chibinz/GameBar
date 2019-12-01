use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::barrel_shifter::shift_register;
use crate::memory::Memory;

pub fn decode_execute(cpu: &mut CPU, memory: &mut Memory, instruction: u32)
{
    execute(cpu, memory, decode(instruction));
}

#[inline]
pub fn decode(instruction: u32) -> (bool, bool, bool, bool, bool, bool, u32, u32, u32)
{
    debug_assert_eq!(bits(instruction, 27, 26), 0b01);

    let i      = bit(instruction, 25);
    let p      = bit(instruction, 24);
    let u      = bit(instruction, 23);
    let b      = bit(instruction, 22);
    let w      = bit(instruction, 21);
    let l      = bit(instruction, 20);
    let rn     = bits(instruction, 19, 16);
    let rd     = bits(instruction, 15, 12);
    let offset = bits(instruction, 11, 0);

    (i, p, u, b, w, l, rn, rd, offset)
}

#[inline]
pub fn execute(cpu: &mut CPU, memory: &mut Memory, 
    (i, p, u, b, w, l, rn, rd, offset): (bool, bool, bool, bool, bool, bool, u32, u32, u32))
{
    let noffset = if i {offset} else {shift_register(cpu, offset)};

    let post = cpu.register.r[rn as usize];
    let pre = if u {cpu.register.r[rn as usize] + noffset} 
              else {cpu.register.r[rn as usize] - noffset};

    let address = if p {pre} else {post};
    let data = cpu.register.r[rd as usize];

    // Misaligned word access is not handled
    match (b, l)
    {
        (false, false) => memory.store8(address, data as u8),
        (true, false)  => memory.store32(address, data),
        (false, true)  => cpu.register.r[rd as usize] = memory.load8(address) as u32,
        (true, true)   => cpu.register.r[rd as usize] = memory.load32(address)
    }

    if w || !p
    {
        cpu.register.r[rn as usize] = pre;

        // Privileged write back bit not handled
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn byte_transfer()
    {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        memory.store8(0x02000001, 0xff);
        cpu.register.r[0] = 0x02000000;

        execute(&mut cpu, &mut memory, (true, true, true, true, true, true, 0, 1, 1));
        assert_eq!(cpu.register.r[1], 0xff);
        assert_eq!(cpu.register.r[0], 0x02000001);

        execute(&mut cpu, &mut memory, (true, true, false, false, false, false, 0, 1, 1));
        assert_eq!(memory.load8(0x02000000), 0xff);
    }

    #[test]
    fn word_transfer()
    {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        memory.store32(0x02000000, 0xdeadbeef);
        cpu.register.r[0] = 0x02000000;

        execute(&mut cpu, &mut memory, (true, false, true, true, false, true, 0, 1, 4));
        assert_eq!(cpu.register.r[1], 0xdeadbeef);
        assert_eq!(cpu.register.r[0], 0x02000004);


        cpu.register.r[1] = 0;
        execute(&mut cpu, &mut memory, (true, true, false, true, true, false, 0, 1, 4));
        assert_eq!(memory.load32(0x02000000), 0);
        assert_eq!(cpu.register.r[0], 0x02000000);
    }
}