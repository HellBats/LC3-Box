mod image;
mod operations;
mod vm;
mod traps;
mod input_buffering;
mod hardware;

use std::env;
use std::process::exit;

use input_buffering::{disable_input_buffering, restore_input_buffering};
use hardware::Registers;
use operations::OPCODE_TABLE;
use crate::vm::VM;

fn main() {
    let mut vm = VM::new();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        exit(0);
    }

    print!("{}", vm.state_read());
    for i in 1..args.len() {
        image::read_image(&args[i], &mut vm);
    }

    vm.register_write(Registers::R_PC as usize, 0x3000);
    disable_input_buffering();

    // Main VM loop
    while vm.state_read() {
        let instruction_register = vm.register_read(Registers::R_PC.into());
        vm.register_write(Registers::R_PC.into(), instruction_register + 1); // PC incremented
        let instruction: u16 = vm.memory_read(instruction_register);
        let opcode = instruction >> 12;
        OPCODE_TABLE[opcode as usize](instruction, &mut vm);
    }

    // Restore terminal state before exit
    restore_input_buffering();
    println!("\nVM exited cleanly.");
}
