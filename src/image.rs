use std::fs;
use crate::vm::VM;
use crate::hardware::MEMORY_MAX;

pub fn read_image(path:&str, vm:&mut VM)
{
    let buffer =  match fs::read(path)
    {
        Ok(file) => file,

        Err(err) => panic!("file not found")
    };
    /* swap to little endian */
    let base = (buffer[0] as u16)<<8 | buffer[1] as u16;


    let mut max_read = (MEMORY_MAX as u16 - base) as usize;
    let mut buffer_pointer = 2;
    
    while (max_read > buffer_pointer && buffer_pointer<buffer.len()) 
    {
        let value = (buffer[buffer_pointer] as u16)<<8 | buffer[buffer_pointer+1] as u16;
        vm.memory_write(base, value);
        buffer_pointer+=2;
    }
}
