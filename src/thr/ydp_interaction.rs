use std::fs::File;
use std::io::prelude::Read;

pub fn load_file(file_name: &str) -> Vec<u8> {
    let header: [u8; 18] = [
        0xF0, 0x43, 0x7D, 0x00,
        0x02,
        0x0C,
        b'D', b'T', b'A', b'1', b'A', b'l', b'l', b'P', 0x00, 0x00, 0x7F, 0x7F
    ];

    let mut file_content = [0; 265];

    let mut file_handler = match File::open(file_name) {
        Ok(file_handler) => file_handler,
        Err(e) => panic!(e.to_string())
    };

    match file_handler.read(&mut file_content) {
        Ok(x) => println!("read {} bytes from {}", x, file_name),
        Err(e) => panic!(e)
    };

    let file_content = &file_content[9..];

    let hcrc: u32 = header[6..].iter().map(|&x| x as u32).sum();
    let fcrc: u32 = file_content.iter().map(|&x| x as u32).sum();
    let mut crc: u32 = hcrc + fcrc;
    crc = (!crc + 1) & 0x7F;

    let mut sysex:Vec<u8> = Vec::new();
    sysex.extend_from_slice(&header);
    sysex.extend_from_slice(&file_content);
    sysex.push(crc as u8);
    sysex.push(0xF7);

    sysex
}
