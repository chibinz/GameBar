pub struct Flash {
    flash: Vec<u8>,
    command: Vec<(usize, u8)>,
    bank: usize,
    state: u32,
    id: bool,
    erase: bool,
}

impl Flash {
    pub fn new() -> Self {
        Self {
            flash: vec![0; 128 * 1024],
            command: Vec::new(),
            bank: 0,
            state: 0,
            id: false,
            erase: false,
        }
    }
}

impl util::Bus for Flash {
    fn load8(&self, address: usize) -> u8 {
        if self.id {
            if address == 0 {
                return 0xc2;
            } else if address == 1 {
                return 0x09;
            }
        }

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
                    self.id = true;
                }
                // Exit ID mode
                [(0x5555, 0xf0)] => {
                    self.id = false;
                }
                // Erase command
                [(0x5555, 0x80)] => {
                    self.erase = true;
                    self.state = 0;
                    self.command.clear();
                    return;
                }
                // Erase entire chip
                [(0x5555, 0x10)] => {
                    if self.erase {
                        self.flash.iter_mut().for_each(|b| *b = 0xff);
                    }
                }
                // Erase 4 KB sector
                [(n, 0x30)] => {
                    if self.erase {
                        let a = self.bank + *n;
                        self.flash[a..a + 4096].iter_mut().for_each(|b| *b = 0xff);
                    }
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
            self.erase = false;
            self.state = 0;
            self.command.clear();
        } else {
            self.erase = false;
            self.state = 0;
            self.command.clear();
        }
    }
}
