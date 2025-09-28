use crate::hardware::{self, CondtionalFlags, Registers,Memory_Mapped_registers::MR_KBDR,Memory_Mapped_registers::MR_KBSR};
use std::io;
use std::io::Read;
use crate::input_buffering::check_key;
pub struct VM {
    state:bool,
    memory: [u16; hardware::MEMORY_MAX], // 65,536 memory locations
    registers: [u16; hardware::Registers::R_COUNT as usize],   // R0-R7, PC, COND
}

impl VM
{
    pub fn new() -> Self
    {
        Self
        {
            state: true,
            memory: [0;hardware::MEMORY_MAX],
            registers: [0; hardware::Registers::R_COUNT as usize]
        }
    }

    pub fn memory_read(&mut self,address:u16) -> u16
    {
        if (address == (MR_KBSR as u16))
        {
            if (check_key())
            {
                self.memory_write(MR_KBSR as u16,  1<< 15);
                let mut buffer = [0u8; 1];
                // read one byte from stdin (line-buffered: requires Enter)
                io::stdin().read_exact(&mut buffer).unwrap();
                self.memory_write(MR_KBDR as u16,  buffer[0] as u16);
            }
            else
            {
                self.memory_write(MR_KBSR as u16,  0);
            }
        }
        self.memory[address as usize]
    }

    pub fn memory_write(&mut self,address:u16,value:u16)
    {
        self.memory[address as usize] = value;
    }
    pub fn register_read(&mut self,register:usize) -> u16
    {
        if register>Registers::R_COUNT as usize {panic!("Invalid register given");}
        self.registers[register]
    }

    pub fn register_write(&mut self,register:usize,value:u16)
    {
        if register>Registers::R_COUNT as usize {panic!("Invalid register given");}
        self.registers[register] = value;
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
    pub fn state_change(&mut self)
    {
        self.state = !self.state
    }
    pub fn state_read(&self) -> bool
    {
        self.state
    }
}