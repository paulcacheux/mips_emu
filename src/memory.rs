#[derive(Debug, Clone)]
pub struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    pub fn new(mem_size: usize) -> Memory {
        Memory {
            data: vec![0; mem_size]
        }
    }

    pub fn get_byte(&self, index: u32) -> u8 {
        let index = index as usize;

        self.data[index]
    }

    pub fn set_byte(&mut self, index: u32, byte: u8) {
        let index = index as usize;

        self.data[index] = byte;
    }

    pub fn get_half_word(&self, index: u32) -> u16 {
        let index = index as usize;

        let _0 = self.data[index + 0] as u16;
        let _1 = self.data[index + 1] as u16;

        _0 | (_1 << 8)
    }

    pub fn set_half_word(&mut self, index: u32, half_word: u16) {
        let index = index as usize;

        self.data[index + 0] = half_word as u8;
        self.data[index + 1] = (half_word >> 8) as u8;
    }

    pub fn get_word(&self, index: u32) -> u32 {
        let index = index as usize;

        let _0 = self.data[index + 0] as u32;
        let _1 = self.data[index + 1] as u32;
        let _2 = self.data[index + 2] as u32;
        let _3 = self.data[index + 3] as u32;

        _0 | (_1 << 8) | (_2 << 16) | (_3 << 24)
    } 

    pub fn set_word(&mut self, index: u32, word: u32) {
        let index = index as usize;

        self.data[index + 0] = word as u8;
        self.data[index + 1] = (word >> 8) as u8;
        self.data[index + 2] = (word >> 16) as u8;
        self.data[index + 3] = (word >> 24) as u8;
    }
}
