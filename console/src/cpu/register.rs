use crate::cpu::CPU;

use PSRBit::*;
use PSRMode::*;

/// Bits 31 - 28, 7 - 5 of Current Program Status Register
#[allow(dead_code)]
pub enum PSRBit {
    N = 31, // Sign Flag
    Z = 30, // Zero Flag
    C = 29, // Carry Flag
    V = 28, // Overflow Flag
    // Bits 27 - 8 are reserved
    I = 7, // IRQ Disable
    F = 6, // FIQ Disable
    T = 5, // State Bit, Thumb/Arm
           // Bits 4 - 0 are mode bits
}

/// Operating Mode
pub enum PSRMode {
    User = 0b10000,
    FIQ = 0b10001,
    IRQ = 0b10010,
    Supervisor = 0b10011,
    Abort = 0b10111,
    Undefined = 0b11011,
    System = 0b11111,
}

impl CPU {
    #[inline]
    pub fn get_cpsr(&self) -> u32 {
        self.cpsr
    }

    /// Set defined bits of CPSR.
    /// If parameter f is set, transfer only the flag bits.
    /// Reserved bits of CPSR are kept intact.
    #[inline]
    pub fn set_cpsr(&mut self, r: u32, f: bool) {
        if f {
            // Set condition code flags only
            let mask = 0xf0000000;
            self.cpsr &= !mask;
            self.cpsr |= r & mask;
        } else {
            // Save state and switch mode
            let mode = CPU::get_mode(r & 0b11111);
            self.save_state();
            self.switch_bank(mode);

            // Change control bits
            self.cpsr = r;
        };
    }

    #[inline]
    pub fn restore_cpsr(&mut self) {
        self.set_cpsr(self.spsr, false);
    }

    #[inline]
    pub fn get_cpsr_bit(&self, bit: PSRBit) -> bool {
        self.cpsr >> (bit as u32) & 1 == 1
    }

    #[inline]
    pub fn set_cpsr_bit(&mut self, bit: PSRBit, t: bool) {
        let b = bit as u32;

        self.cpsr &= !(1 << b);
        self.cpsr |= (t as u32) << b;
    }

    /// Get SPSR of current mode
    #[inline]
    pub fn get_spsr(&mut self) -> u32 {
        self.spsr
    }

    /// Set defined bits of SPSR of current mode.
    /// If parameter f is set, transfer only the flag bits.
    /// Reserved bits of SPSR are kept intact.
    #[inline]
    pub fn set_spsr(&mut self, r: u32, f: bool) {
        let mask = if f { 0xf0000000 } else { 0xf00000ff };

        self.spsr &= !mask;
        self.spsr |= r & mask;
    }

    #[inline]
    pub fn get_mode(mbits: u32) -> PSRMode {
        match mbits {
            0b10000 => User,
            0b10001 => FIQ,
            0b10010 => IRQ,
            0b10011 => Supervisor,
            0b10111 => Abort,
            0b11011 => Undefined,
            0b11111 => System,
            _ => panic!("Invalid PSR Mode\n"),
        }
    }

    pub fn save_state(&mut self) {
        let mode = CPU::get_mode(self.cpsr & 0b11111);

        match mode {
            User => {
                self.bank[5] = self.r[13];
                self.bank[6] = self.r[14]
            }
            System => {
                self.bank[5] = self.r[13];
                self.bank[6] = self.r[14]
            }
            FIQ => {
                self.bank[12] = self.r[13];
                self.bank[13] = self.r[14];
                self.bank[14] = self.spsr
            }
            Supervisor => {
                self.bank[15] = self.r[13];
                self.bank[16] = self.r[14];
                self.bank[17] = self.spsr
            }
            Abort => {
                self.bank[18] = self.r[13];
                self.bank[19] = self.r[14];
                self.bank[20] = self.spsr
            }
            IRQ => {
                self.bank[21] = self.r[13];
                self.bank[22] = self.r[14];
                self.bank[23] = self.spsr
            }
            Undefined => {
                self.bank[24] = self.r[13];
                self.bank[25] = self.r[14];
                self.bank[26] = self.spsr
            }
        };

        if let FIQ = mode {
            for i in 0..5 {
                self.bank[i + 7] = self.r[i + 8];
            }
        } else {
            for i in 0..5 {
                self.bank[i] = self.r[i + 8];
            }
        }
    }

    pub fn switch_bank(&mut self, mode: PSRMode) {
        match mode {
            User => {
                self.r[13] = self.bank[5];
                self.r[14] = self.bank[6];
            }
            System => {
                self.r[13] = self.bank[5];
                self.r[14] = self.bank[6];
            }
            FIQ => {
                self.r[13] = self.bank[12];
                self.r[14] = self.bank[13];
                self.spsr = self.bank[14];
            }
            Supervisor => {
                self.r[13] = self.bank[15];
                self.r[14] = self.bank[16];
                self.spsr = self.bank[17];
            }
            Abort => {
                self.r[13] = self.bank[18];
                self.r[14] = self.bank[19];
                self.spsr = self.bank[20];
            }
            IRQ => {
                self.r[13] = self.bank[21];
                self.r[14] = self.bank[22];
                self.spsr = self.bank[23];
            }
            Undefined => {
                self.r[13] = self.bank[24];
                self.r[14] = self.bank[25];
                self.spsr = self.bank[26];
            }
        };

        if let FIQ = mode {
            for i in 0..5 {
                self.r[i + 8] = self.bank[i + 7];
            }
        } else {
            for i in 0..5 {
                self.r[i + 8] = self.bank[i];
            }
        }
    }

    /// Return true if condition is satified
    pub fn check_condition(&self, condition: u32) -> bool {
        match condition {
            0b0000 => self.get_cpsr_bit(Z),                          // EQ
            0b0001 => !self.get_cpsr_bit(Z),                         // NE
            0b0010 => self.get_cpsr_bit(C),                          // CS
            0b0011 => !self.get_cpsr_bit(C),                         // CC
            0b0100 => self.get_cpsr_bit(N),                          // MI
            0b0101 => !self.get_cpsr_bit(N),                         // PL
            0b0110 => self.get_cpsr_bit(V),                          // VS
            0b0111 => !self.get_cpsr_bit(V),                         // VC
            0b1000 => self.get_cpsr_bit(C) && !self.get_cpsr_bit(Z), // HI
            0b1001 => !self.get_cpsr_bit(C) || self.get_cpsr_bit(Z), // LS
            0b1010 => self.get_cpsr_bit(N) == self.get_cpsr_bit(V),  // GE
            0b1011 => self.get_cpsr_bit(N) != self.get_cpsr_bit(V),  // LT
            0b1100 => !self.get_cpsr_bit(Z) && (self.get_cpsr_bit(N) == self.get_cpsr_bit(V)), // GT
            0b1101 => self.get_cpsr_bit(Z) || (self.get_cpsr_bit(N) != self.get_cpsr_bit(V)), // LE
            0b1110 => true,
            _ => panic!("Invalid Condition Field!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn get_cpsr_bit() {
        let mut cpu = CPU::new();
        cpu.cpsr = 0b100000;

        assert_eq!(cpu.get_cpsr_bit(PSRBit::T), true);
    }

    #[test]
    fn set_cpsr_bit() {
        let mut cpu = CPU::new();
        cpu.set_cpsr_bit(PSRBit::F, true);

        assert_eq!(cpu.cpsr.bit(6), true);
    }

    #[test]
    fn check_condition() {
        use PSRBit::*;

        let mut cpu = CPU::new();

        cpu.set_cpsr_bit(Z, true);
        assert!(cpu.check_condition(0b0000));

        cpu.set_cpsr_bit(C, true);
        assert!(cpu.check_condition(0b0010));

        cpu.set_cpsr_bit(N, true);
        assert!(cpu.check_condition(0b0100));

        cpu.set_cpsr_bit(V, true);
        assert!(cpu.check_condition(0b0110));

        assert!(cpu.check_condition(0b1010));

        cpu.set_cpsr_bit(Z, false);
        assert!(cpu.check_condition(0b1100));
    }
}
