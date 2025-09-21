
fn sign_extension(val:u16, bit_count:u8) -> u16
{
    (val>>bit_count-1)<<15 | val
}
fn OP_BR(){}
fn OP_ADD(){}
fn OP_LD(){}
fn OP_ST(){}
fn OP_JSR(){}
fn OP_AND(){}
fn OP_LDR(){}
fn OP_STR(){}
fn OP_RTI(){}
fn OP_NOT(){}
fn OP_LDI(){}
fn OP_STI(){}
fn OP_JMP(){}
fn OP_RES(){}
fn OP_LEA(){}
fn OP_TRAP(){}



pub const OPCODE_TABLE:[fn();16] = 
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