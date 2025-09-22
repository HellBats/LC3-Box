use crate::hardware::{self, CondtionalFlags, Registers};
pub struct VM {
    pub memory: [u16; hardware::MEMORY_MAX], // 65,536 memory locations
    pub registers: [u16; hardware::Registers::R_COUNT as usize],   // R0-R7, PC, COND
}

impl VM
{
    pub fn new() -> Self
    {
        Self
        {
            memory: [0;hardware::MEMORY_MAX],
            registers: [0; hardware::Registers::R_COUNT as usize]
        }
    }

    pub fn memory_read(&mut self,address:u16) -> u16
    {
        self.memory[address as usize]
    }

    pub fn memory_write(&mut self,address:u16,value:u16)
    {
        self.memory[address as usize] = value;
    }
    pub fn register_read(&mut self,register:usize) -> u16
    {
        if register>Registers::R_COUNT as usize {panic!("Invalid register given");}
        self.registers[register as usize]
    }

    pub fn register_write(&mut self,register:usize,value:u16)
    {
        if register>Registers::R_COUNT as usize {panic!("Invalid register given");}
        self.registers[register as usize] = value;
    }
    pub fn update_flags(&mut self,register_no:usize)
    {
        self.registers[Registers::R_COND as usize]  = if self.registers[register_no] == 0
        {
            CondtionalFlags::FL_ZRO as u16
        }
        else if self.registers[register_no]>>15 ==1
        {
            CondtionalFlags::FL_NEG as u16
        }
        else
        {
            CondtionalFlags::FL_POS as u16
        }
    }
}