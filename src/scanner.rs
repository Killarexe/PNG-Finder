#[derive(Debug)]
pub struct Scanner {
    bytes: Vec<u8>,
    index: usize,
    current_byte: u8
}

impl Scanner {
    pub fn new(bytes: Vec<u8>) -> Self {
        let current_byte: u8 = bytes[0].clone();
        Self{
            bytes,
            index: 0,
            current_byte
        }
    }

    fn advence(&mut self) {
        if let Some(byte) = self.bytes.get(self.index + 1) {
            self.index += 1;
            self.current_byte = *byte;
        }
    }

    fn peek(&self, offset: usize) -> u8 {
        if let Some(byte) = self.bytes.get(self.index + offset) {
            return *byte;
        }
        0u8
    }

    pub fn scan(&mut self) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        let mut current_result: (usize, usize) = (0, 0);
        while !self.is_end_of_file() {
            if self.is_png_header() {
                current_result.0 = self.index;
            }else if current_result.0 < self.index && self.is_png_tail() {
                current_result.1 = self.index + 7;
                result.push(current_result);
            }
            self.advence();
        }
        result
    }

    fn is_png_header(&self) -> bool {
        self.current_byte == 0x89
        && self.peek(1) == 0x50
        && self.peek(2) == 0x4E
        && self.peek(3) == 0x47
    }

    fn is_png_tail(&self) -> bool {
        self.current_byte == 0x49
        && self.peek(1) == 0x45
        && self.peek(2) == 0x4E
        && self.peek(3) == 0x44
        && self.peek(4) == 0xAE
        && self.peek(5) == 0x42
        && self.peek(6) == 0x60
        && self.peek(7) == 0x82
    }

    fn is_end_of_file(&self) -> bool {
        self.index >= self.bytes.len() - 1
    }
}
