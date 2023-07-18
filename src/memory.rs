pub struct Memory([u8; 0xFFFF]);

impl Memory {
    pub fn init() -> Self {
        Self([0; 0xFFFF])
    }

    pub fn read(&self, address: u16) -> u8 {
        self.0[usize::from(address)]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.0[usize::from(address)] = value;
    }
}
