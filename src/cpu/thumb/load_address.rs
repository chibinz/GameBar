use crate::util::*;
use crate::cpu::CPU;

#[inline]
pub fn interpret(cpu: &mut CPU, instruction: u16)
{
    execute(cpu, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (bool, u32, u32)
{
    let sp = instruction.bit(11);
    let rd = instruction.bits(10, 8);
    let word8 = instruction.bits(7, 0);

    (sp, rd, word8)
}
 
#[inline]
fn execute(cpu: &mut CPU, (sp, rd, word8): (bool, u32, u32))
{   
    if sp
    {
        cpu.r[rd as usize] = cpu.r[13] + (word8 << 2);
    }
    else
    {
        // Bit 1 of PC is forced to 0.
        // The value of the PC will be 4 bytes greater than the address 
        // of the instruction before bit 1 is forced to 0.
        cpu.r[rd as usize] = (cpu.r[15] & 0xfffffffc) + (word8 << 2);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    
    #[test]
    fn load_address()
    {
        let mut cpu = CPU::new();

        cpu.r[13] = 0xffffff00;
        execute(&mut cpu, (true, 0, 0b00111111));
        assert_eq!(cpu.r[0], 0xfffffffc);
    }
}