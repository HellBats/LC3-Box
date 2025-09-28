use std::fs;
use crate::vm::VM;
use crate::hardware::MEMORY_MAX;


pub fn read_image(path: &str, vm: &mut VM) {
    let buffer = match fs::read(path) {
        Ok(file) => file,
        Err(_) => panic!("file not found"),
    };

    if buffer.len() < 2 {
        panic!("invalid image file: too small");
    }

    // first two bytes: origin (big endian in LC-3 format)
    let base: u16 = ((buffer[0] as u16) << 8) | buffer[1] as u16;

    // number of 16-bit words we can actually read
    let words_available = ((buffer.len() - 2) / 2) as u16;

    for i in 0..words_available {
        let idx = 2 + (i as usize * 2);
        let value = ((buffer[idx] as u16) << 8) | (buffer[idx + 1] as u16);
        vm.memory_write(base + i, value);
    }
}

