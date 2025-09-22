#![allow(warnings)]
use std::{env, process::exit};
mod hardware;
mod image;
mod operations;
mod vm;

use hardware::Registers;
use operations::OPCODE_TABLE;

use crate::vm::VM;
fn main() {
    let mut vm = VM::new();
    let args:Vec<String> = env::args().collect();
    if args.len()>1
    {
        exit(0);
    } 
    image::read_image(&args[0]);
    vm.register_write(Registers::R_PC as usize, 0x3000);
    loop
    {
        let instruction_register =  vm.register_read(Registers::R_PC as usize);
        let instruction:u16 = vm.memory_read(instruction_register);
        let opcode = instruction>>12;
        OPCODE_TABLE[opcode as usize]();
    }
}
