use bit_flag::*;

#[derive(Copy, Clone)]
pub struct Register(pub u8);

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Register> for u8 {
    fn from(value: Register) -> u8 {
        value.0
    }
}

#[bit_flag]
impl Register {
    #[flag_mask]
    pub const BIT_7: u8 = 0b1000_0000;

    #[flag_mask]
    pub const BIT_6: u8 = 0b0100_0000;

    #[value_mask]
    pub const BITS_543: u8 = 0b0011_1000;

    #[value_mask(MyMode)]
    pub const BITS_20: u8 = 0b0000_0101;

    #[flag_mask]
    pub const BIT_1: u8 = 0b0000_0010;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
pub enum MyMode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

impl From<U2> for MyMode {
    fn from(value: U2) -> Self {
        match value.to_bits() {
            [false, false] => Self::Mode0,
            [true, false] => Self::Mode1,
            [false, true] => Self::Mode2,
            [true, true] => Self::Mode3,
        }
    }
}

impl From<MyMode> for U2 {
    fn from(value: MyMode) -> Self {
        match value {
            MyMode::Mode0 => U2::new(0),
            MyMode::Mode1 => U2::new(1),
            MyMode::Mode2 => U2::new(2),
            MyMode::Mode3 => U2::new(3),
        }
    }
}

#[test]
fn test_bit_flag() {
    let reg = Register(0b1010_1010);

    assert_eq!(reg.get_bit_7(), true);
    assert_eq!(reg.get_bit_6(), false);
    assert_eq!(reg.get_bits_543(), U3::from(0b101));
    assert_eq!(reg.get_bits_20(), MyMode::Mode0);
    assert_eq!(reg.get_bit_1(), true);

    let mut reg = Register(0b0101_0101);

    assert_eq!(reg.get_bit_7(), false);
    assert_eq!(reg.get_bit_6(), true);
    assert_eq!(reg.get_bits_543(), U3::from(0b010));
    assert_eq!(reg.get_bits_20(), MyMode::Mode3);
    assert_eq!(reg.get_bit_1(), false);

    reg.set_bits_543(U3::from(0b111));

    assert_eq!(reg.get_bit_7(), false);
    assert_eq!(reg.get_bit_6(), true);
    assert_eq!(reg.get_bits_543(), U3::from(0b111));
    assert_eq!(reg.get_bits_20(), MyMode::Mode3);
    assert_eq!(reg.get_bit_1(), false);

    reg.set_bit_6(false);

    assert_eq!(reg.get_bit_7(), false);
    assert_eq!(reg.get_bit_6(), false);
    assert_eq!(reg.get_bits_543(), U3::from(0b111));
    assert_eq!(reg.get_bits_20(), MyMode::Mode3);
    assert_eq!(reg.get_bit_1(), false);

    reg.set_bit_1(true);

    assert_eq!(reg.get_bit_7(), false);
    assert_eq!(reg.get_bit_6(), false);
    assert_eq!(reg.get_bits_543(), U3::from(0b111));
    assert_eq!(reg.get_bits_20(), MyMode::Mode3);
    assert_eq!(reg.get_bit_1(), true);

    reg.set_bits_20(MyMode::Mode2);

    assert_eq!(reg.get_bit_7(), false);
    assert_eq!(reg.get_bit_6(), false);
    assert_eq!(reg.get_bits_543(), U3::from(0b111));
    assert_eq!(reg.get_bits_20(), MyMode::Mode2);
    assert_eq!(reg.get_bit_1(), true);
}
