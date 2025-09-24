
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