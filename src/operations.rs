
use crate::hardware;
use crate::vm::VM;
use hardware::Registers;


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
        vm.register_write(Registers::R_PC.into(), ((pc as i16) + (offset as i16)) as u16);
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
        let ans = (vm.register_read(source_register1) as i16 +
        immidiate as i16) as u16;
        vm.register_write(destination_register, ans);
    }
    else
    {
        let source_register2 = (inst & 7) as usize;
        let ans = (vm.register_read(source_register1) as i16 +
        vm.register_read(source_register2) as i16) as u16;
        vm.register_write(destination_register, ans);
    }
    vm.update_flags(destination_register);
}


fn OP_LD(inst:u16,vm:&mut VM){}
fn OP_ST(inst:u16,vm:&mut VM){}
fn OP_JSR(inst:u16,vm:&mut VM){}


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


fn OP_LDR(inst:u16,vm:&mut VM){}
fn OP_STR(inst:u16,vm:&mut VM){}
fn OP_RTI(inst:u16,vm:&mut VM){}
fn OP_NOT(inst:u16,vm:&mut VM){}
fn OP_LDI(inst:u16,vm:&mut VM){}
fn OP_STI(inst:u16,vm:&mut VM){}
fn OP_JMP(inst:u16,vm:&mut VM){}
fn OP_RES(inst:u16,vm:&mut VM){}
fn OP_LEA(inst:u16,vm:&mut VM){}
fn OP_TRAP(inst:u16,vm:&mut VM){}



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