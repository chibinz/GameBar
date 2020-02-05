use crate::cpu::CPU;
use crate::cpu::register::PSRMode::Supervisor;

#[inline]
pub fn interpret(cpu: &mut CPU)
{   
    let lr = cpu.r[15] - if cpu.in_thumb_mode() {2} else {4};
    let spsr = cpu.get_cpsr();

    cpu.set_cpsr(Supervisor as u32, false);    
    
    cpu.set_spsr(spsr, false);
    cpu.r[14] = lr;
    cpu.r[15] = 0x08;
    cpu.flush();
}