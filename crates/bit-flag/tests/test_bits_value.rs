use bit_flag::*;

#[test]
fn test_bits_value() {
    let u3 = U3::from_bits([true, false, true]);
    assert_eq!(0b101u8, u3.into());
    assert_eq!([true, false, true], u3.to_bits());

    let u5 = U5::from(0b11100u8);
    assert_eq!(0b11100u8, u5.into());
    assert_eq!([false, false, true, true, true], u5.to_bits());
}
