
use super::*;

fn as_i16(v: u16) -> i16 
{
    v as i16
}
#[test]
fn test_sign_extension() 
{
    // 2-bit signed values: 00=0, 01=1, 10=-2, 11=-1
    assert_eq!(as_i16(sign_extension(0b00, 2)), 0);
    assert_eq!(as_i16(sign_extension(0b01, 2)), 1);
    assert_eq!(as_i16(sign_extension(0b10, 2)), -2);
    assert_eq!(as_i16(sign_extension(0b11, 2)), -1);
    // 3-bit signed values: 000=0, 001=1, 010=2, 011=3, 100=-4, 101=-3, 110=-2, 111=-1
    assert_eq!(as_i16(sign_extension(0b000, 3)), 0);
    assert_eq!(as_i16(sign_extension(0b011, 3)), 3);
    assert_eq!(as_i16(sign_extension(0b100, 3)), -4);
    assert_eq!(as_i16(sign_extension(0b111, 3)), -1);

    // 8-bit signed values: check boundaries
    assert_eq!(as_i16(sign_extension(0x7F, 8)), 127);   // 0111_1111
    assert_eq!(as_i16(sign_extension(0x80, 8)), -128);  // 1000_0000
    assert_eq!(as_i16(sign_extension(0xFF, 8)), -1);    // 1111_1111
    // 1-bit signed values: 0 = 0, 1 = -1
    assert_eq!(as_i16(sign_extension(0b0, 1)), 0);
    assert_eq!(as_i16(sign_extension(0b1, 1)), -1);
}
