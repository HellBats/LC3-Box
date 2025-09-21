
use std::{env, process::exit};
mod hardware;
mod image;
mod operations;

use hardware::Registers;
use operations::OPCODE_TABLE;
fn main() {
    let mut memory = [0;hardware::MEMORY_MAX];
    let mut registers:[u16;Registers::R_COUNT as usize] = [0;Registers::R_COUNT as usize];
    let args:Vec<String> = env::args().collect();
    if args.len()>1
    {
        exit(0);
    } 
    image::read_image(&args[0]);
    registers[Registers::R_PC as usize] = 0x3000;
    loop
    {
        let instruction:u16 = memory[registers[Registers::R_PC as usize] as usize];
        let opcode = instruction>>12;
        OPCODE_TABLE[opcode as usize]();
    }
}
