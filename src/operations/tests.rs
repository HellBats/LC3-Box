
use super::*;
fn as_i16(v: u16) -> i16 
{
    v as i16
}

// ---------------- Update Flags OPERATION ----------------

#[test]
fn test_update_flags_zero() {
    let mut vm = VM::new();
    vm.register_write(0, 0); // R0 = 0
    vm.update_flags(0);

    assert_eq!(
        vm.register_read(Registers::R_COND.into()),
        CondtionalFlags::FL_ZRO as u16,
        "Zero flag not set correctly"
    );
}

#[test]
fn test_update_flags_positive() {
    let mut vm = VM::new();
    vm.register_write(1, 123); // R1 = positive number
    vm.update_flags(1);

    assert_eq!(
        vm.register_read(Registers::R_COND.into()),
        CondtionalFlags::FL_POS as u16,
        "Positive flag not set correctly"
    );
}

#[test]
fn test_update_flags_negative() {
    let mut vm = VM::new();
    vm.register_write(2, 0x8000); // R2 = 1000_0000_0000_0000 (MSB=1)
    vm.update_flags(2);

    assert_eq!(
        vm.register_read(Registers::R_COND.into()),
        CondtionalFlags::FL_NEG as u16,
        "Negative flag not set correctly"
    );
}

#[test]
fn test_update_flags_negative_custom_value() {
    let mut vm = VM::new();
    vm.register_write(3, 0xFFFF); // R3 = -1 in two’s complement
    vm.update_flags(3);

    assert_eq!(
        vm.register_read(Registers::R_COND.into()),
        CondtionalFlags::FL_NEG as u16,
        "Negative flag not set for -1"
    );
}


// ---------------- SIGN EXTENSION ----------------

#[test]
fn sign_extension_2bit() {
    // 2-bit signed values: 00=0, 01=1, 10=-2, 11=-1
    assert_eq!(as_i16(sign_extension(0b00, 2)), 0);
    assert_eq!(as_i16(sign_extension(0b01, 2)), 1);
    assert_eq!(as_i16(sign_extension(0b10, 2)), -2);
    assert_eq!(as_i16(sign_extension(0b11, 2)), -1);
}

#[test]
fn sign_extension_3bit() {
    // 3-bit signed values: 000=0, ..., 100=-4, 111=-1
    assert_eq!(as_i16(sign_extension(0b000, 3)), 0);
    assert_eq!(as_i16(sign_extension(0b011, 3)), 3);
    assert_eq!(as_i16(sign_extension(0b100, 3)), -4);
    assert_eq!(as_i16(sign_extension(0b111, 3)), -1);
}

#[test]
fn sign_extension_8bit_boundaries() {
    assert_eq!(as_i16(sign_extension(0x7F, 8)), 127);   // 0111_1111
    assert_eq!(as_i16(sign_extension(0x80, 8)), -128);  // 1000_0000
    assert_eq!(as_i16(sign_extension(0xFF, 8)), -1);    // 1111_1111
}

#[test]
fn sign_extension_1bit() {
    assert_eq!(as_i16(sign_extension(0b0, 1)), 0);
    assert_eq!(as_i16(sign_extension(0b1, 1)), -1);
}

// ---------------- ADD OPERATION ----------------

#[test]
fn add_with_immediate() {
    let mut vm = VM::new();
    let inst = 0b0001_000_001_1_00101; // ADD R0, R1, #5
    vm.register_write(1, 10);
    OP_ADD(inst, &mut vm);
    assert_eq!(vm.register_read(0), 15);
}

#[test]
fn add_with_registers() {
    let mut vm = VM::new();
    let inst = 0b0001_010_011_0_00_100; // ADD R2, R3, R4
    vm.register_write(3, 20);
    vm.register_write(4, 22);
    OP_ADD(inst, &mut vm);
    assert_eq!(vm.register_read(2), 42);
}

#[test]
fn add_with_registers_negative_values() {
    let mut vm = VM::new();
    let inst = 0b0001_010_011_0_00_100; // ADD R2, R3, R4
    vm.register_write(3, (-20 as i16) as u16);
    vm.register_write(4, (-22 as i16) as u16);
    OP_ADD(inst, &mut vm);
    assert_eq!(vm.register_read(2), (-42 as i16) as u16);
}

#[test]
fn add_with_negative_immediate() {
    let mut vm = VM::new();
    let inst = 0b0001_101_110_1_11111; // ADD R5, R6, #-1
    vm.register_write(6, 5);
    OP_ADD(inst, &mut vm);
    assert_eq!(vm.register_read(5), 4);
}

// ---------------- AND OPERATION ----------------

#[test]
fn and_with_immediate() {
    let mut vm = VM::new();
    let inst = 0b0001_000_001_1_01101; // ADD R0, R1, #5
    vm.register_write(1, 12);
    OP_AND(inst, &mut vm);
    assert_eq!(vm.register_read(0), 12);
}

#[test]
fn and_with_registers() {
    let mut vm = VM::new();
    let inst = 0b0001_010_011_0_00_100; // ADD R2, R3, R4
    vm.register_write(3, 15);
    vm.register_write(4, 9);
    OP_AND(inst, &mut vm);
    assert_eq!(vm.register_read(2), 9);
}


// ---------------- BR OPERATION ----------------

 #[test]
fn test_br_taken_zero_flag_forward() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x3000);
    vm.register_write(Registers::R_COND.into(), 0b010); // Z

    // BRz with offset = +5
    let inst = 0b0000_010_000000101;
    OP_BR(inst, &mut vm);

    assert_eq!(vm.register_read(Registers::R_PC.into()), 0x3005);
}

#[test]
fn test_br_taken_negative_flag_backward() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x3000);
    vm.register_write(Registers::R_COND.into(), 0b100); // N

    // BRn with offset = -4 (111111100 in 9 bits)
    let inst = 0b0000_100_111111100;
    OP_BR(inst, &mut vm);

    assert_eq!(vm.register_read(Registers::R_PC.into()), 0x2FFC);
}

#[test]
fn test_br_taken_positive_flag_forward() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x3000);
    vm.register_write(Registers::R_COND.into(), 0b001); // P

    // BRp with offset = +2
    let inst = 0b0000_001_000000010;
    OP_BR(inst, &mut vm);

    assert_eq!(vm.register_read(Registers::R_PC.into()), 0x3002);
}

#[test]
fn test_br_not_taken_condition_mismatch() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x3000);
    vm.register_write(Registers::R_COND.into(), 0b001); // P

    // BRz with offset = +5 (but cond= P, so mismatch)
    let inst = 0b0000_010_000000101;
    OP_BR(inst, &mut vm);

    // PC unchanged
    assert_eq!(vm.register_read(Registers::R_PC.into()), 0x3000);
}

#[test]
fn test_br_multiple_conditions() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x3000);
    vm.register_write(Registers::R_COND.into(), 0b100); // N

    // BRnz with offset = +3, and cond=N → should branch
    let inst = 0b0000_110_000000011;
    OP_BR(inst, &mut vm);

    assert_eq!(vm.register_read(Registers::R_PC.into()), 0x3003);
}

#[test]
fn test_br_offset_zero() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x3000);
    vm.register_write(Registers::R_COND.into(), 0b010); // Z

    // BRz with offset = 0
    let inst = 0b0000_010_000000000;
    OP_BR(inst, &mut vm);

    // PC stays same (3000)
    assert_eq!(vm.register_read(Registers::R_PC.into()), 0x3000);
}

#[test]
fn test_br_wraparound() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x0001);
    vm.register_write(Registers::R_COND.into(), 0b100); // N

    // BRn with offset = -2 (111111110 in 9 bits)
    let inst = 0b0000_100_111111110;
    OP_BR(inst, &mut vm);

    // PC should wrap around to 0xFFFF
    assert_eq!(vm.register_read(Registers::R_PC.into()), 0xFFFF);
}

// ---------------- JMP & RET OPERATION ----------------

#[test]
fn test_jmp()
{
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x3000);
    vm.register_write(Registers::R_R1.into(), 0x4000); // Jump to this address

    let inst = 0b1100_000_001_000000;
    OP_JMP(inst, &mut vm);
    assert_eq!(vm.register_read(Registers::R_PC.into()),0x4000);
}

#[test]
fn test_jmp_ret()
{
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x4000);
    vm.register_write(Registers::R_R7.into(), 0x3000); // Jump to this address

    let inst = 0b1100_000_111_000000;
    OP_JMP(inst, &mut vm);
    assert_eq!(vm.register_read(Registers::R_PC.into()),0x3000);
}

#[test]
fn test_jmp_wrong_register()
{
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x4000);
    vm.register_write(Registers::R_R6.into(), 0x3000); // Jump to this address

    let inst = 0b1100_000_001_000000;
    OP_JMP(inst, &mut vm);
    assert_ne!(vm.register_read(Registers::R_PC.into()),0x3000);
}



// ---------------- JSR & JSRR OPERATION ----------------

#[test]
fn test_jsr_positive_offset() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x3000);

    // JSR with offset = +5
    // opcode=0100, bit[11]=1, offset=0000000101
    let inst = 0b0100_1_00000000101;

    OP_JSR(inst, &mut vm);

    // R7 should contain old PC
    assert_eq!(vm.register_read(Registers::R_R7.into()), 0x3000);
    // PC should be old PC + 5
    assert_eq!(vm.register_read(Registers::R_PC.into()), 0x3005);
}

#[test]
fn test_jsr_negative_offset() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x3000);

    // JSR with offset = -2 (11111111110 in 11-bit)
    let inst = 0b0100_1_11111111110;

    OP_JSR(inst, &mut vm);

    assert_eq!(vm.register_read(Registers::R_R7.into()), 0x3000);
    assert_eq!(vm.register_read(Registers::R_PC.into()), 0x2FFE);
}

#[test]
fn test_jsrr() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x3000);
    vm.register_write(3, 0x4000); // baseR=R3

    // JSRR with baseR=3
    // opcode=0100, bit[11]=0, baseR=011
    let inst = 0b0100_0_000_011_000000;

    OP_JSR(inst, &mut vm);

    assert_eq!(vm.register_read(Registers::R_R7.into()), 0x3000);
    assert_eq!(vm.register_read(Registers::R_PC.into()), 0x4000);
}


// ---------------- LD OPERATION ----------------

#[test]
fn test_ld_zero_offset() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x3000);
    vm.memory_write(0x3000, 0x1234);

    // inst = opcode(0010) + DR=R1 + offset=0
    let inst: u16 = 0b0010_001_000000000; 

    OP_LD(inst, &mut vm);

    assert_eq!(vm.register_read(Registers::R_R1.into()), 0x1234);
    assert_eq!(vm.register_read(Registers::R_COND.into()), CondtionalFlags::FL_POS as u16);
}

#[test]
fn test_ld_positive_offset() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x3000);
    vm.memory_write(0x3005, 0xABCD);

    // inst = opcode(0010) + DR=R2 + offset=+5
    let inst: u16 = 0b0010_010_000000101; 

    OP_LD(inst, &mut vm);

    assert_eq!(vm.register_read(Registers::R_R2.into()), 0xABCD);
    assert_eq!(vm.register_read(Registers::R_COND.into()), CondtionalFlags::FL_NEG as u16); 
    // 0xABCD has MSB=1
}

#[test]
fn test_ld_negative_offset() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x3005);
    vm.memory_write(0x3000, 0x0000);

    // inst = opcode(0010) + DR=R3 + offset=-5
    let inst: u16 = 0b0010_011_111111011; // -5 in 9-bit two’s complement

    OP_LD(inst, &mut vm);

    assert_eq!(vm.register_read(Registers::R_R3.into()), 0x0000);
    assert_eq!(vm.register_read(Registers::R_COND.into()), CondtionalFlags::FL_ZRO as u16);
}

#[test]
fn test_ld_updates_flags_positive_value() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_PC.into(), 0x3100);
    vm.memory_write(0x3101, 42);

    // inst = opcode(0010) + DR=R4 + offset=+1
    let inst: u16 = 0b0010_100_000000001;

    OP_LD(inst, &mut vm);

    assert_eq!(vm.register_read(Registers::R_R4.into()), 42);
    assert_eq!(vm.register_read(Registers::R_COND.into()), CondtionalFlags::FL_POS as u16);
}

// ---------------- NOT OPERATION ----------------

#[test]
fn test_not() {
    let mut vm = VM::new();
    vm.register_write(Registers::R_R1.into(), 0x00FF);

    let inst: u16 = 0b1001_010_001_000001;

    OP_NOT(inst, &mut vm);

    assert_eq!(vm.register_read(Registers::R_R2.into()), 0xFF00);
}


// ---------------- LDI OPERATION ----------------


#[test]
fn test_ldi_basic() {
    let mut vm = VM::new();

    // Set PC to 0x3000
    vm.register_write(Registers::R_PC.into(), 0x3000);

    // Instruction: LDI R1, PCoffset9 = +1
    // opcode=1010, DR=001, offset=000000001
    let inst = 0b1010_001_000000001;

    // Memory[0x3000 + 1] = 0x3050 (the indirect pointer)
    vm.memory_write(0x3001, 0x3050);

    // Memory[0x3050] = 0xABCD (the final value)
    vm.memory_write(0x3050, 0xABCD);

    OP_LDI(inst, &mut vm);

    // R1 should now contain the final value 0xABCD
    assert_eq!(vm.register_read(1), 0xABCD);
}

#[test]
fn test_ldi_negative_offset() {
    let mut vm = VM::new();

    // PC = 0x3100
    vm.register_write(Registers::R_PC.into(), 0x3100);

    // Instruction: LDI R2, offset = -1
    // offset field = 0b111111111 = -1
    let inst = 0b1010_010_111111111;

    // Memory[0x3100 - 1] = 0x4000
    vm.memory_write(0x30FF, 0x4000);

    // Memory[0x4000] = 0x1234
    vm.memory_write(0x4000, 0x1234);

    OP_LDI(inst, &mut vm);

    assert_eq!(vm.register_read(2), 0x1234);
}

#[test]
fn test_ldi_updates_flags() {
    let mut vm = VM::new();

    vm.register_write(Registers::R_PC.into(), 0x3000);

    // LDI R3, offset = +1
    let inst = 0b1010_011_000000001;

    // Indirect pointer
    vm.memory_write(0x3001, 0x4000);

    // Final value = 0
    vm.memory_write(0x4000, 0x0000);

    OP_LDI(inst, &mut vm);

    assert_eq!(vm.register_read(3), 0);
    assert_eq!(
        vm.register_read(Registers::R_COND.into()),
        CondtionalFlags::FL_ZRO as u16
    );
}



// ---------------- LDR OPERATION ----------------


#[test]
fn test_ldr_basic_positive_offset() {
    let mut vm = VM::new();

    // Instruction: LDR R1, R2, #3
    // opcode=0110, DR=001, BaseR=010, offset6=000011
    let inst = 0b0110_001_010_000011;

    // Base register (R2) = 0x3000
    vm.register_write(2, 0x3000);

    // Memory[0x3000 + 3] = 0xABCD
    vm.memory_write(0x3003, 0xABCD);

    OP_LDR(inst, &mut vm);

    assert_eq!(vm.register_read(1), 0xABCD);
}

#[test]
fn test_ldr_negative_offset() {
    let mut vm = VM::new();

    // Instruction: LDR R4, R5, #-2
    // offset6 = 111110 (sign-extended = -2)
    let inst = 0b0110_100_101_111110;

    // Base register (R5) = 0x4000
    vm.register_write(5, 0x4000);

    // Memory[0x4000 - 2] = 0x1234
    vm.memory_write(0x3FFE, 0x1234);

    OP_LDR(inst, &mut vm);

    assert_eq!(vm.register_read(4), 0x1234);
}

#[test]
fn test_ldr_updates_flags_zero() {
    let mut vm = VM::new();

    // Instruction: LDR R6, R7, #0
    // offset6 = 000000
    let inst = 0b0110_110_111_000000;

    // Base register (R7) = 0x5000
    vm.register_write(7, 0x5000);

    // Memory[0x5000] = 0x0000
    vm.memory_write(0x5000, 0x0000);

    OP_LDR(inst, &mut vm);

    assert_eq!(vm.register_read(6), 0x0000);
    assert_eq!(
        vm.register_read(Registers::R_COND.into()),
        CondtionalFlags::FL_ZRO as u16
    );
}

#[test]
fn test_ldr_updates_flags_negative() {
    let mut vm = VM::new();

    // Instruction: LDR R0, R1, #1
    let inst = 0b0110_000_001_000001;

    // Base register (R1) = 0x6000
    vm.register_write(1, 0x6000);

    // Memory[0x6001] = 0xFFFF (-1 in two's complement)
    vm.memory_write(0x6001, 0xFFFF);

    OP_LDR(inst, &mut vm);

    assert_eq!(vm.register_read(0), 0xFFFF);
    assert_eq!(
        vm.register_read(Registers::R_COND.into()),
        CondtionalFlags::FL_NEG as u16
    );
}




// ---------------- LEA OPERATION ----------------



#[test]
fn test_lea_basic_positive_offset() {
    let mut vm = VM::new();

    // Set PC to 0x3000
    vm.register_write(Registers::R_PC.into(), 0x3000);

    // Instruction: LEA R1, PC+5
    // opcode=1110, DR=001, offset9=000000101
    let inst = 0b1110_001_000000101;

    OP_LEA(inst, &mut vm);

    // R1 should hold PC + 5
    assert_eq!(vm.register_read(1), 0x3005);
}

#[test]
fn test_lea_negative_offset() {
    let mut vm = VM::new();

    vm.register_write(Registers::R_PC.into(), 0x3100);

    // Instruction: LEA R2, PC-4
    // offset9 = 111111100 (-4 after sign extension)
    let inst = 0b1110_010_111111100;

    OP_LEA(inst, &mut vm);

    assert_eq!(vm.register_read(2), 0x30FC);
}

#[test]
fn test_lea_updates_flags_zero() {
    let mut vm = VM::new();

    // PC = 0
    vm.register_write(Registers::R_PC.into(), 0x0000);

    // LEA R3, offset = 0
    let inst = 0b1110_011_000000000;

    OP_LEA(inst, &mut vm);

    assert_eq!(vm.register_read(3), 0x0000);
    assert_eq!(
        vm.register_read(Registers::R_COND.into()),
        CondtionalFlags::FL_ZRO as u16
    );
}

#[test]
fn test_lea_updates_flags_positive() {
    let mut vm = VM::new();

    vm.register_write(Registers::R_PC.into(), 0x4000);

    // LEA R4, offset = +1
    let inst = 0b1110_100_000000001;

    OP_LEA(inst, &mut vm);

    assert_eq!(vm.register_read(4), 0x4001);
    assert_eq!(
        vm.register_read(Registers::R_COND.into()),
        CondtionalFlags::FL_POS as u16
    );
}

#[test]
fn test_lea_updates_flags_negative() {
    let mut vm = VM::new();

    vm.register_write(Registers::R_PC.into(), 0x8000);

    // LEA R5, offset = +0 (so result is 0x8000)
    // offset9 = 0b000000000
    let inst = 0b1110_101_000000000;

    OP_LEA(inst, &mut vm);

    assert_eq!(vm.register_read(5), 0x8000); // MSB=1 → negative
    assert_eq!(
        vm.register_read(Registers::R_COND.into()),
        CondtionalFlags::FL_NEG as u16
    );
}


// ---------------- ST OPERATION ----------------


#[test]
fn test_st_basic_positive_offset() {
    let mut vm = VM::new();

    // PC = 0x3000
    vm.register_write(Registers::R_PC.into(), 0x3000);

    // R1 = 0xABCD
    vm.register_write(1, 0xABCD);

    // Instruction: ST R1, PC+2
    // opcode=0011, SR=001, offset9=000000010
    let inst = 0b0011_001_000000010;

    OP_ST(inst, &mut vm);

    // Expect value stored at memory[0x3002] = 0xABCD
    assert_eq!(vm.memory_read(0x3002), 0xABCD);
}

#[test]
fn test_st_negative_offset() {
    let mut vm = VM::new();

    vm.register_write(Registers::R_PC.into(), 0x3100);

    // R2 = 0x1234
    vm.register_write(2, 0x1234);

    // Instruction: ST R2, PC-3
    // offset9 = 111111101 (-3 in two’s complement)
    let inst = 0b0011_010_111111101;

    OP_ST(inst, &mut vm);

    // Expect value stored at memory[0x30FD] = 0x1234
    assert_eq!(vm.memory_read(0x30FD), 0x1234);
}

#[test]
fn test_st_zero_offset() {
    let mut vm = VM::new();

    vm.register_write(Registers::R_PC.into(), 0x4000);
    vm.register_write(3, 0xDEAD);

    // Instruction: ST R3, PC+0
    let inst = 0b0011_011_000000000;

    OP_ST(inst, &mut vm);

    // Expect memory[0x4000] = 0xDEAD
    assert_eq!(vm.memory_read(0x4000), 0xDEAD);
}

#[test]
fn test_st_overwrites_existing_value() {
    let mut vm = VM::new();

    vm.register_write(Registers::R_PC.into(), 0x5000);
    vm.register_write(4, 0xBEEF);

    // Pre-fill memory at 0x5005
    vm.memory_write(0x5005, 0x1111);

    // Instruction: ST R4, PC+5
    let inst = 0b0011_100_000000101;

    OP_ST(inst, &mut vm);

    // Value should be overwritten with 0xBEEF
    assert_eq!(vm.memory_read(0x5005), 0xBEEF);
}



// ---------------- STI OPERATION ----------------


#[test]
fn test_sti_basic_positive_offset() {
    let mut vm = VM::new();

    // PC = 0x3000
    vm.register_write(Registers::R_PC.into(), 0x3000);

    // R1 = 0xABCD
    vm.register_write(1, 0xABCD);

    // memory[0x3002] = 0x4000 → this is the target address
    vm.memory_write(0x3002, 0x4000);

    // Instruction: STI R1, PC+2
    // opcode=1011, SR=001, offset9=000000010
    let inst = 0b1011_001_000000010;

    OP_STI(inst, &mut vm);

    // Expect memory[0x4000] = 0xABCD
    assert_eq!(vm.memory_read(0x4000), 0xABCD);
}

#[test]
fn test_sti_negative_offset() {
    let mut vm = VM::new();

    vm.register_write(Registers::R_PC.into(), 0x3100);

    // R2 = 0x1234
    vm.register_write(2, 0x1234);

    // memory[0x30FD] = 0x2000 → target address
    vm.memory_write(0x30FD, 0x2000);

    // Instruction: STI R2, PC-3
    // offset9 = 111111101 (-3 in two’s complement)
    let inst = 0b1011_010_111111101;

    OP_STI(inst, &mut vm);

    // Expect memory[0x2000] = 0x1234
    assert_eq!(vm.memory_read(0x2000), 0x1234);
}

#[test]
fn test_sti_zero_offset() {
    let mut vm = VM::new();

    vm.register_write(Registers::R_PC.into(), 0x4000);
    vm.register_write(3, 0xDEAD);

    // memory[0x4000] = 0x5000 → target address
    vm.memory_write(0x4000, 0x5000);

    // Instruction: STI R3, PC+0
    let inst = 0b1011_011_000000000;

    OP_STI(inst, &mut vm);

    // Expect memory[0x5000] = 0xDEAD
    assert_eq!(vm.memory_read(0x5000), 0xDEAD);
}

#[test]
fn test_sti_overwrites_existing_value() {
    let mut vm = VM::new();

    vm.register_write(Registers::R_PC.into(), 0x5000);
    vm.register_write(4, 0xBEEF);

    // memory[0x5005] = 0x6000 → target address
    vm.memory_write(0x5005, 0x6000);

    // Pre-fill target memory
    vm.memory_write(0x6000, 0x1111);

    // Instruction: STI R4, PC+5
    let inst = 0b1011_100_000000101;

    OP_STI(inst, &mut vm);

    // Value should be overwritten with 0xBEEF
    assert_eq!(vm.memory_read(0x6000), 0xBEEF);
    
}



// ---------------- STR OPERATION ----------------



#[test]
fn test_str_basic_positive_offset() {
    let mut vm = VM::new();

    // R2 = base register = 0x3000
    vm.register_write(2, 0x3000);

    // R1 = source register = 0xABCD
    vm.register_write(1, 0xABCD);

    // Instruction: STR R1, R2, #5
    // opcode=0111, SR=001, BaseR=010, offset6=000101
    let inst = 0b0111_001_010_000101;

    OP_STR(inst, &mut vm);

    // Expect memory[0x3000 + 5] = 0xABCD
    assert_eq!(vm.memory_read(0x3005), 0xABCD);
}

#[test]
fn test_str_negative_offset() {
    let mut vm = VM::new();

    // Base register = R3 = 0x3100
    vm.register_write(3, 0x3100);

    // Source register = R4 = 0x1234
    vm.register_write(4, 0x1234);

    // Instruction: STR R4, R3, #-3
    // offset6 = 111101 (-3 in 6-bit two’s complement)
    let inst = 0b0111_100_011_111101;

    OP_STR(inst, &mut vm);

    // Expect memory[0x3100 - 3] = 0x1234
    assert_eq!(vm.memory_read(0x30FD), 0x1234);
}

#[test]
fn test_str_zero_offset() {
    let mut vm = VM::new();

    // Base register = R5 = 0x4000
    vm.register_write(5, 0x4000);

    // Source register = R6 = 0xDEAD
    vm.register_write(6, 0xDEAD);

    // Instruction: STR R6, R5, #0
    let inst = 0b0111_110_101_000000;

    OP_STR(inst, &mut vm);

    // Expect memory[0x4000] = 0xDEAD
    assert_eq!(vm.memory_read(0x4000), 0xDEAD);
}

#[test]
fn test_str_overwrites_existing_value() {
    let mut vm = VM::new();

    // Base register = R7 = 0x5000
    vm.register_write(7, 0x5000);

    // Source register = R0 = 0xBEEF
    vm.register_write(0, 0xBEEF);

    // Pre-fill target memory with some value
    vm.memory_write(0x5002, 0x1111);

    // Instruction: STR R0, R7, #2
    let inst = 0b0111_000_111_000010;

    OP_STR(inst, &mut vm);

    // Value should be overwritten with 0xBEEF
    assert_eq!(vm.memory_read(0x5002), 0xBEEF);
}


// ---------------- TRAP OPERATION ----------------