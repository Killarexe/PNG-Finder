use std::{fs::OpenOptions, io::Write};

struct Finder {
    stop: bool,
    index: usize,
    input: Vec<u8>,
    images_found: u32,
}

impl Finder {
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            stop: false,
            index: 0,
            input: input,
            images_found: 0,
        }
    }

    pub fn advence(&mut self) {
        if self.index < self.input.len() - 1 {
            self.index += 1;
        } else {
            self.stop = true;
        }
    }

    pub fn peek(&mut self, offset: usize) -> u8 {
        if self.index + offset < self.input.len() {
            return self.input[self.index + offset].clone();
        }
        return 0u8;
    }

    pub fn previous(&mut self, offset: usize) -> u8 {
        if self.index as i32 - offset as i32 >= 0 {
            return self.input[self.index - offset].clone();
        }
        return 0u8;
    }

    pub fn is_end_png(&mut self) -> bool {
        self.previous(7) == 0x49
            && self.previous(6) == 0x45
            && self.previous(5) == 0x4E
            && self.previous(4) == 0x44
            && self.previous(3) == 0xAE
            && self.previous(2) == 0x42
            && self.previous(1) == 0x60
            && self.input[self.index] == 0x82
    }

    pub fn read(&mut self) {
        while !self.stop {
            let current_value = self.input[self.index].clone();
            if current_value == 0x89
                && self.peek(1) == 0x50
                && self.peek(2) == 0x4E
                && self.peek(3) == 0x47
            {
                let mut output_file: Vec<u8> = Vec::new();
                while !(self.is_end_png()) {
                    output_file.push(self.input[self.index].clone());
                    self.advence();
                }
                output_file.push(0x82u8);
                let mut output_file_result = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(format!("image_result/image{}.png", self.images_found))
                    .expect("Failed to create file.");
                output_file_result
                    .write_all(&output_file)
                    .expect("Failed to write");
                self.images_found += 1;
            } else {
                self.advence();
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: png_finder file");
        return;
    }
    let file_contents: Vec<u8> = std::fs::read(args[1].to_string()).expect("Failed to read file");
    let mut finder: Finder = Finder::new(file_contents);
    std::fs::create_dir("image_result").unwrap_or_default();
    finder.read();
    println!("Found {} images", finder.images_found);
}
