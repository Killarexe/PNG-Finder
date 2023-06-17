use std::{io::Write, fs::File};

struct Finder {
    stop: bool,
    index: usize,
    input: Vec<u8>,
    images_found: u32,
}

struct FileSignature{
    header: Vec<u8>,
    tail: Vec<u8>,
    format: String,
    name: String
}

impl FileSignature {
    pub fn new(header: Vec<u8>, tail: Vec<u8>, format: &str, name: &str) -> Self{
        Self{
            header: header,
            tail: tail,
            format: String::from(format),
            name: String::from(name)
        }
    }
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

    pub fn peek(&self, offset: usize) -> u8 {
        if self.index + offset < self.input.len() {
            return self.input[self.index + offset].clone();
        }
        return 0u8;
    }

    pub fn _previous(&self, offset: usize) -> u8 {
        if self.index as i32 - offset as i32 >= 0 {
            return self.input[self.index - offset].clone();
        }
        return 0u8;
    }

    pub fn find(&self, value: &Vec<u8>) -> bool{
        for i in 0..value.len(){
            if value[i] != self.peek(i) || self.stop{
                return false;
            }
        }
        true
    }

    pub fn read(&mut self) {
        let mut index_start: usize = 0;
        let file_signatures:Vec<FileSignature> = vec![
            FileSignature::new(
                vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
                vec![0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82],
                "png",
                "image_png"
            ),
            //FileSignature::new(vec![0xFF, 0xD8, 0xFF, 0xE0], vec![0xFF, 0xD9], "jpg", "image_jpg"),
            //FileSignature::new(vec![0xFF, 0xD8, 0xFF, 0xE1], vec![0xFF, 0xD9], "jpg", "image_jpg")
        ];
        while !self.stop {
            for signature in &file_signatures{
                if self.find(&signature.header){
                    index_start = self.index.clone();
                }
                if self.find(&signature.tail){
                    self.create_file(index_start, self.index + signature.tail.len(), &signature.name, &signature.format);
                }
            }
            self.advence();
        }
    }

    pub fn create_file(&mut self, start: usize, end: usize, name: &str, format: &str){
        let mut file: File = File::create(format!("result/{}_{}.{}", name, self.images_found, format)).expect("Failed to create file.");
        file.write(&self.input[start..end]).expect("Failed to write file");
        self.images_found += 1;
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
    std::fs::create_dir("result").unwrap_or_default();
    finder.read();
    println!("Found {} images", finder.images_found);
}
