use crate::u12::U12;

pub struct Memory {
    mem: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            mem: Vec::with_capacity(U12::size()),
        }
    }

    fn read(&self, addr: U12) -> u8 {
        self.mem[addr.to_usize()]
    }

    fn write(&mut self, addr: U12, val: u8) {
        self.mem[addr.to_usize()] = val;
    }
}
