use crate::cpu::CPU;
use crate::cpu::register::PSRMode::Supervisor;

#[inline]
pub fn interpret(cpu: &mut CPU)
{
    cpu.r[15] = 0x08;
    cpu.flush();

    cpu.set_spsr(cpu.get_cpsr(), false);
    cpu.set_cpsr(Supervisor as u32, false);
}