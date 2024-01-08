use crate::scanner::Scanner;

fn get_results(input: Vec<u8>) -> Vec<(usize, usize)> {
    let mut scanner: Scanner = Scanner::new(input); 
    scanner.scan()
}

fn get_bytes(input: Vec<u8>, outputs: Vec<(usize, usize)>) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    for output in outputs.iter() {
        result.push(input[(output.0)..output.1].to_vec());
    }
    result
}

#[test]
fn single_png_file_not_hidden() {
    let contents: Vec<u8> = vec![
        0x89, 0x50, 0x4E, 0x47,
        0xAA, 0xBB, 0xCC, 0xDD,
        0xEE, 0xFF, 0x00, 0x11,
        0x49, 0x45, 0x4E, 0x44,
        0xAE, 0x42, 0x60, 0x82
    ];
    let results: Vec<(usize, usize)> = get_results(contents.clone());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], (0, contents.len()));
    assert_eq!(get_bytes(contents.clone(), results)[0], contents);
}

#[test]
fn multiple_png_file_not_hidden() {
    let first_png: Vec<u8> = vec![
        0x89, 0x50, 0x4E, 0x47,
        0xAA, 0xBB, 0xCC, 0xDD,
        0xEE, 0xFF, 0x00, 0x11,
        0x49, 0x45, 0x4E, 0x44,
        0xAE, 0x42, 0x60, 0x82
    ];
    let second_png: Vec<u8> = vec![
        0x89, 0x50, 0x4E, 0x47,
        0x12, 0x34, 0x56, 0x78,
        0x49, 0x45, 0x4E, 0x44,
        0xAE, 0x42, 0x60, 0x82
    ];
    let mut contents: Vec<u8> = first_png.clone();
    contents.extend(second_png.clone());
    let results: Vec<(usize, usize)> = get_results(contents.clone());
    assert_eq!(results.len(), 2);
    assert_eq!(results[0], (0, first_png.len()));
    assert_eq!(results[1], (first_png.len(), first_png.len() + second_png.len()));

    let bytes: Vec<Vec<u8>> = get_bytes(contents, results);
    assert_eq!(bytes[0], first_png);
    assert_eq!(bytes[1], second_png);
}

#[test]
fn single_png_file_hidden() {
    let png_file: Vec<u8> = vec![
        0x89, 0x50, 0x4E, 0x47,
        0xAA, 0xBB, 0xCC, 0xDD,
        0xEE, 0xFF, 0x00, 0x11,
        0x49, 0x45, 0x4E, 0x44,
        0xAE, 0x42, 0x60, 0x82
    ];
    let mut contents: Vec<u8> = Vec::new();
    contents.extend(vec![
        0x51, 0x13, 0x13, 0x12,
        0x12, 0x98, 0x00, 0xFF
    ]);
    let start_png: usize = contents.len();
    contents.extend(png_file.clone());
    contents.extend(vec![0x98, 0x13, 0x49, 0x54]);
    let results: Vec<(usize, usize)> = get_results(contents.clone());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], (start_png, start_png + png_file.len()));
    assert_eq!(get_bytes(contents.clone(), results)[0], png_file);
}

#[test]
fn multiple_png_file_hidden() {
    let first_png: Vec<u8> = vec![
        0x89, 0x50, 0x4E, 0x47,
        0xAA, 0xBB, 0xCC, 0xDD,
        0xEE, 0xFF, 0x00, 0x11,
        0x49, 0x45, 0x4E, 0x44,
        0xAE, 0x42, 0x60, 0x82
    ];
    let second_png: Vec<u8> = vec![
        0x89, 0x50, 0x4E, 0x47,
        0x12, 0x34, 0x56, 0x78,
        0x49, 0x45, 0x4E, 0x44,
        0xAE, 0x42, 0x60, 0x82
    ];
    let mut contents: Vec<u8> = Vec::new();
    contents.extend(0..123);

    let start_first_png: usize = contents.len();
    contents.extend(first_png.clone());
    contents.extend(13..255);

    let start_second_png: usize = contents.len();

    contents.extend(second_png.clone());

    let results: Vec<(usize, usize)> = get_results(contents.clone());
    assert_eq!(results.len(), 2);
    assert_eq!(results[0], (start_first_png, start_first_png + first_png.len()));
    assert_eq!(results[1], (start_second_png, start_second_png + second_png.len()));

    let bytes: Vec<Vec<u8>> = get_bytes(contents, results);
    assert_eq!(bytes[0], first_png);
    assert_eq!(bytes[1], second_png);
}
