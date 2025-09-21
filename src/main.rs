
use std::{env, process::exit};
mod hardware;
mod image;


fn main() {
    let mut memory = [0;hardware::MEMORY_MAX];
    let mut registers:[u16;hardware::Registers::R_COUNT as usize];
    let args:Vec<String> = env::args().collect();
    if args.len()>1
    {
        exit(0);
    } 
    image::read_image(&args[0]);
    println!("Hello, world!");
}
