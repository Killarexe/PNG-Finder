use std::{path::PathBuf, fs::{OpenOptions, File, self}, io::{Write, ErrorKind}};

use clap::Parser;

mod args;
mod scanner;

fn main() {
    let args: args::Args = args::Args::parse();
    let dir_output: PathBuf = args.get_dir_output();

    match fs::read(args.file_input) {
        Ok(file_contents) => {
            process(file_contents, dir_output);
        },
        Err(e) => {
            println!("{:#?}", e);
        }
    }
}

fn process(bytes: Vec<u8>, dir_output: PathBuf) {
    let mut scanner: scanner::Scanner = scanner::Scanner::new(bytes.clone());
    let results: Vec<(usize, usize)> = scanner.scan();
    drop(scanner);
    println!("Found {} png images.", results.len());
    match fs::create_dir(dir_output.clone()) {
        Ok(_) => {},
        Err(err) => {
            if err.kind() != ErrorKind::AlreadyExists {
                panic!("Failed to create dir...");
            }
        }
    }
    for (index, result) in results.iter().enumerate() {
        let file_output: Vec<u8> = bytes[(result.0)..result.1].to_vec();

        let file_name: String = format!("result_{}.png", index);
        let mut dir_path: PathBuf = dir_output.clone();
        dir_path.push(file_name);

        let file_path: &str = dir_path.to_str().expect("Failed to create file due to bad path...");

        let mut file_options: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .expect("Failed to create file options...");
        file_options.write_all(&file_output).expect("Failed to write to file...");
    }
}
