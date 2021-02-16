pub struct Flash {
    flash: Vec<u8>,
    command: Vec<(usize, u8)>,
    bank: usize,
    state: u32,
    tmp: [u8; 2],
}

impl Flash {
    pub fn new() -> Self {
        Self {
            flash: vec![0; 128 * 1024],
            command: Vec::new(),
            bank: 0,
            state: 0,
            tmp: [0; 2],
        }
    }
}

impl util::Bus for Flash {
    fn load8(&self, address: usize) -> u8 {
        self.flash[self.bank + address]
    }

    fn store8(&mut self, address: usize, value: u8) {
        if (self.state, address, value) == (0, 0x5555, 0xaa) {
            self.state = 1
        } else if (self.state, address, value) == (1, 0x2aaa, 0x55) {
            self.state = 2
        } else if self.state == 2 {
            self.command.push((address, value));

            match self.command.as_slice() {
                // Enter ID mode
                [(0x5555, 0x90)] => {
                    let a = self.bank + address;
                    self.tmp.copy_from_slice(&self.flash[a..a + 2]);
                    self.flash[a..a + 2].copy_from_slice(&[0xc2, 0x09]);
                }
                // Exit ID mode
                [(0x5555, 0xf0)] => {
                    let a = self.bank + address;
                    self.flash[a..a + 2].copy_from_slice(&self.tmp);
                }
                // Erase command
                [(0x5555, 0x80)] => {}
                // Erase entire chip
                [(0x5555, 0x10)] => {
                    self.flash.iter_mut().for_each(|b| *b = 0xff);
                }
                // Erase 4 KB sector
                [(n, 0x30)] => {
                    let a = self.bank + *n;
                    self.flash[a..a + 4096].iter_mut().for_each(|b| *b = 0xff);
                }
                // Look ahead
                [(0x5555, 0xa0)] => {
                    return;
                }
                // Single data write
                [(0x5555, 0xa0), (addr, value)] => {
                    self.flash[self.bank + addr] = *value;
                }
                // Look ahead
                [(0x5555, 0xb0)] => {
                    return;
                }
                // Bank switching
                [(0x5555, 0xb0), (0, bank)] => {
                    self.bank = (*bank as usize) << 16;
                }
                _ => {}
            }
            self.state = 0;
            self.command.clear();
        } else {
            self.state = 0;
            self.command.clear();
        }
    }
}
