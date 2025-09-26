
use crate::hardware;
use crate::vm::VM;
use crate::traps::Traps;
use hardware::Registers;
use hardware::CondtionalFlags;
use std::io;
use std::io::Read;

fn sign_extension(val: u16, bit_count: u8) -> u16
{
    let shift = 16 - bit_count;
    (((val << shift) as i16) >> shift) as u16
}


fn OP_BR(inst:u16,vm:&mut VM)
{
    let offset = sign_extension(inst, 9);
    if ((inst>>9 & vm.register_read(Registers::R_COND.into()))!=0)
    {
        let pc = vm.register_read(Registers::R_PC.into());
        vm.register_write(Registers::R_PC.into(), ((pc as i32) + (offset as i32)) as u16);
    }
}


fn OP_ADD(inst:u16,vm:&mut VM)
{
    let destination_register = (inst>>9 & 7 )as usize ;
    let immidiate_bit = inst>>5 & 1;
    let source_register1 = (inst>>6 & 7) as usize;
    if immidiate_bit > 0
    {
        let immidiate = sign_extension(inst, 5);
        let ans = (vm.register_read(source_register1) as i32 +
        immidiate as i32) as u16;
        vm.register_write(destination_register, ans);
    }
    else
    {
        let source_register2 = (inst & 7) as usize;
        let ans = (vm.register_read(source_register1) as i32 +
        vm.register_read(source_register2) as i32) as u16;
        vm.register_write(destination_register, ans);
    }
    vm.update_flags(destination_register);
}


fn OP_LD(inst:u16,vm:&mut VM)
{
    let destination_register = (inst>>9 & 7 ) as usize;
    let offset = sign_extension(inst, 9) as i32;
    let pc = vm.register_read(Registers::R_PC.into());
    let value = vm.memory_read((pc as i32 + offset) as u16);
    vm.register_write(destination_register , value);
    vm.update_flags(destination_register);
}


fn OP_ST(inst:u16,vm:&mut VM)
{
    let source_register = (inst>>9 & 7 ) as usize;
    let offset = sign_extension(inst, 9) as i32;
    let pc = vm.register_read(Registers::R_PC.into());
    let value = vm.register_read(source_register);
    vm.memory_write((pc as i32 + offset) as u16,value);
}


fn OP_JSR(inst:u16,vm:&mut VM)
{
    let program_counter = vm.register_read(Registers::R_PC.into());
    vm.register_write(Registers::R_R7.into(), program_counter);

    if ((inst >> 11) & 1) == 1 {
        // JSR: PC-relative (11-bit offset)
        let offset = sign_extension(inst, 11) as i32;
        vm.register_write(
            Registers::R_PC.into(),
            (program_counter as i32 + offset) as u16,
        );
    } else {
        // JSRR: register-based
        let base_r = (inst >> 6) & 0x7;
        let target = vm.register_read(base_r as usize);
        vm.register_write(Registers::R_PC.into(), target);
    }
}


fn OP_AND(inst:u16,vm:&mut VM)
{
    let destination_register = (inst>>9 & 7 )as usize ;
    let immidiate_bit = inst>>5 & 1;
    let source_register1 = (inst>>6 & 7) as usize;
    if immidiate_bit > 0
    {
        let immidiate = sign_extension(inst, 5);
        let ans = (vm.register_read(source_register1) as i16 &
        immidiate as i16) as u16;
        vm.register_write(destination_register, ans);
    }
    else
    {
        let source_register2 = (inst & 7) as usize;
        let ans = (vm.register_read(source_register1) as i16 &
        vm.register_read(source_register2) as i16) as u16;
        vm.register_write(destination_register, ans);
    }
    vm.update_flags(destination_register);
}


fn OP_LDR(inst:u16,vm:&mut VM)
{
    let destination_register = (inst>>9 & 7 ) as usize;
    let offset = sign_extension(inst, 6) as i32;
    let base_register = (inst>>6 & 7) as usize;
    let base_address = vm.register_read(base_register);
    let value = vm.memory_read((base_address as i32 + offset) as u16);
    vm.register_write(destination_register , value);
    vm.update_flags(destination_register);
}



fn OP_STR(inst:u16,vm:&mut VM)
{
    let source_register = (inst>>9 & 7 ) as usize;
    let offset = sign_extension(inst, 6) as i32;
    let base_register = (inst>>6 & 7) as usize;
    let base_address = vm.register_read(base_register);
    let value = vm.register_read(source_register);
    vm.memory_write((base_address as i32 + offset) as u16,value);
}


fn OP_RTI(inst:u16,vm:&mut VM)
{
    
}


fn OP_NOT(inst:u16,vm:&mut VM)
{
    let val = !vm.register_read((inst>>6 & 7) as usize);
    vm.register_write((inst>>9 & 7)as usize, val);
    vm.update_flags((inst>>9 & 7)as usize);
}


fn OP_LDI(inst:u16,vm:&mut VM)
{
    let destination_register = (inst>>9 & 7 ) as usize;
    let offset = sign_extension(inst, 9) as i32;
    let pc = vm.register_read(Registers::R_PC.into());
    let address = vm.memory_read((pc as i32 + offset) as u16);
    let value = vm.memory_read(address);
    vm.register_write(destination_register , value);
    vm.update_flags(destination_register);
}


fn OP_STI(inst:u16,vm:&mut VM)
{
    let source_register = (inst>>9 & 7 ) as usize;
    let offset = sign_extension(inst, 9) as i32;
    let pc = vm.register_read(Registers::R_PC.into());
    let address = vm.memory_read((pc as i32 + offset) as u16);
    let value = vm.register_read(source_register);
    vm.memory_write(address,value);
}


fn OP_JMP(inst:u16,vm:&mut VM)
{
    let register = inst>>6 & 7;
    let address = vm.register_read(register.into());
    vm.register_write(Registers::R_PC.into(),address);
}


fn OP_RES(inst:u16,vm:&mut VM)
{
    panic!("Illegal Opcode");
}


fn OP_LEA(inst:u16,vm:&mut VM)
{
    let destination_register = (inst>>9 & 7 ) as usize;
    let offset = sign_extension(inst, 9) as i32;
    let pc = vm.register_read(Registers::R_PC.into());
    let value = (pc as i32 + offset) as u16;
    vm.register_write(destination_register , value);
    vm.update_flags(destination_register);
}


fn OP_TRAP(inst:u16,vm:&mut VM)
{
    let pc = vm.register_read(Registers::R_PC.into());
    vm.register_write(Registers::R_R7.into(), pc);
    let instr = (inst & 0xFF).into(); 
    match (instr)
    {
        Traps::TRAP_GETC => 
        {
            let mut buffer = [0u8; 1];
            // read one byte from stdin (line-buffered: requires Enter)
            io::stdin().read_exact(&mut buffer).unwrap();
            vm.register_write(Registers::R_R0.into(), buffer[0] as u16);

            vm.update_flags(Registers::R_R0.into());
        }
        Traps::TRAP_OUT => 
        {
            let character = vm.register_read(Registers::R_R0.into());
            print!("{character}");
        }
        Traps::TRAP_PUTS => 
        {
            let mut base_address = vm.register_read(Registers::R_R0.into());
            loop 
            {
                let chr = vm.memory_read(base_address) as u8;
                if chr == 0  {break;}
                print!("{}",chr as char);
                base_address+=1;
            }
        }
        Traps::TRAP_IN => 
        {
            println!("Enter a single character: ");
            let mut buffer = [0u8; 1];
            // read one byte from stdin (line-buffered: requires Enter)
            io::stdin().read_exact(&mut buffer).unwrap();
            vm.register_write(Registers::R_R0.into(), buffer[0] as u16);
            vm.update_flags(Registers::R_R0.into());
            print!("{}",buffer[0]);
        }
        Traps::TRAP_PUTSP => 
        {
            let base_address = vm.register_read(Registers::R_R0.into());
            loop 
            {
                let chrs = vm.memory_read(base_address);
                let ch1 = (chrs & 255) as u8;
                if ch1 == 0  {break;}
                print!("{}",ch1 as char);
                let ch2 = (chrs>>8) as u8;
                if ch1 == 0  {break;}
                print!("{}",ch1 as char);
            }
        }
        Traps::TRAP_HALT => 
        {
            println!("VM HAlted");
            vm.state_change();
        }
        Traps::TRAP_INVALID  => {panic!("asd");}
    }
}



pub const OPCODE_TABLE:[fn(inst:u16,vm:&mut VM);16] = 
[
    OP_BR, /* branch */
    OP_ADD,    /* add  */
    OP_LD,     /* load */
    OP_ST,     /* store */
    OP_JSR,    /* jump register */
    OP_AND,    /* bitwise and */
    OP_LDR,    /* load register */
    OP_STR,    /* store register */
    OP_RTI,    /* unused */
    OP_NOT,    /* bitwise not */
    OP_LDI,    /* load indirect */
    OP_STI,    /* store indirect */
    OP_JMP,    /* jump */
    OP_RES,    /* reserved (unused) */
    OP_LEA,    /* load effective address */
    OP_TRAP,  /* execute trap */
];

#[cfg(test)]
mod tests;