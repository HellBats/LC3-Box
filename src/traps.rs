
pub enum Traps
{
    TRAP_GETC = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    TRAP_OUT = 0x21,   /* output a character */
    TRAP_PUTS = 0x22,  /* output a word string */
    TRAP_IN = 0x23,    /* get character from keyboard, echoed onto the terminal */
    TRAP_PUTSP = 0x24, /* output a byte string */
    TRAP_HALT = 0x25,   /* halt the program */
    TRAP_INVALID
}


impl From<u16> for Traps {
    fn from(value: u16) -> Self {
        match value 
        {
            0x20 => Traps::TRAP_GETC,
            0x21 => Traps::TRAP_OUT,    
            0x22 => Traps::TRAP_PUTS,  
            0x23 => Traps::TRAP_IN ,    
            0x24 => Traps::TRAP_PUTSP,  
            0x25 => Traps::TRAP_HALT,
            _ => Traps::TRAP_INVALID
        }
    }
}