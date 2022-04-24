pub struct NetworkMessage {
    pub buffer: Vec<u8>,
    offset: usize,
}

impl NetworkMessage {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { buffer, offset: 0 }
    }

    pub fn skip_bytes(&mut self, n: usize) {
        self.offset += n;
    }

    pub fn get_byte(&mut self) -> u8 {
        let result = self.buffer[self.offset];
        self.offset += 1;
        return result;
    }

    pub fn get_u16(&mut self) -> u16 {
        let result: u16 =
            ((self.buffer[self.offset + 1] as u16) << 8) | (self.buffer[self.offset] as u16);
        self.offset += 2;

        return result;
    }

    pub fn get_u32(&mut self) -> u32 {
        let hi = self.get_u16();
        let lo = self.get_u16();

        return ((lo as u32) << 16) | (hi as u32);
    }

    pub fn get_string(&mut self) -> String {
        let len = self.get_u16() as usize;
        let mut s = String::new();

        for i in 0..len {
            s.push(self.buffer[self.offset + i] as char);
        }

        self.offset += len;

        return s;
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.buffer[self.offset] = byte;
        self.offset += 1;
    }

    pub fn add_u16(&mut self, value: u16) {
        self.buffer[self.offset] = (value & 0xff) as u8;
        self.buffer[self.offset + 1] = ((value >> 8) & 0xff) as u8;
        self.offset += 2;
    }

    pub fn add_u32(&mut self, value: u32) {
        self.add_u16((value & 0xffff) as u16);
        self.add_u16(((value >> 16) & 0xffff) as u16);
    }

    pub fn add_string(&mut self, s: &str) {
        self.add_u16(s.len() as u16);

        for c in s.chars() {
            self.add_byte(c as u8);
        }
    }

    pub fn reset(&mut self) {
        self.offset = 2;
    }
}
