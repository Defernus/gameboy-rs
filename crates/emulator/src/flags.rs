/// **Z** - Zero flag
pub const ZERO_FLAG: u8 = 0b1000_0000;
/// **N** - Subtraction flag (BCD)
pub const SUBTRACT_FLAG: u8 = 0b0100_0000;
/// **H** - Half Carry flag (BCD)
pub const HALF_CARRY_FLAG: u8 = 0b0010_0000;
/// **C** - Carry flag
pub const CARRY_FLAG: u8 = 0b0001_0000;

pub const FLAGS_MASK: u8 = 0b1111_0000;

#[inline(always)]
pub fn set_flag(flags: &mut u8, flag: u8, value: bool) {
    if value {
        *flags |= flag;
    } else {
        *flags &= !flag;
    }
}

#[inline(always)]
pub fn invert_flag(flags: &mut u8, flag: u8) {
    *flags ^= flag;
}

#[inline(always)]
pub fn get_flag(flags: u8, flag: u8) -> bool {
    flags & flag == flag
}

#[inline(always)]
pub fn update_carry_flags_add_u8(flags: &mut u8, a: u8, b: u8) {
    set_flag(
        flags,
        HALF_CARRY_FLAG,
        ((a & 0x0F) + (b & 0x0F)) & (1 << 4) != 0,
    );
    set_flag(flags, CARRY_FLAG, (a as u16 + b as u16) & (1 << 8) != 0);
}

#[inline(always)]
pub fn update_carry_flags_add_u16(flags: &mut u8, a: u16, b: u16) {
    set_flag(
        flags,
        HALF_CARRY_FLAG,
        ((a & 0x0FFF) + (b & 0x0FFF)) & (1 << 12) != 0,
    );
    set_flag(flags, CARRY_FLAG, (a as u32 + b as u32) & (1 << 16) != 0);
}

#[test]
fn test_carry_update() {
    let mut flags: u8 = 0;
    update_carry_flags_add_u8(&mut flags, 0x00, 0x00);
    assert_eq!(get_flag(flags, HALF_CARRY_FLAG), false);
    assert_eq!(get_flag(flags, CARRY_FLAG), false);

    flags = 0;
    update_carry_flags_add_u8(&mut flags, 0x01, 0x02);
    assert_eq!(get_flag(flags, HALF_CARRY_FLAG), false);
    assert_eq!(get_flag(flags, CARRY_FLAG), false);

    flags = 0;
    update_carry_flags_add_u8(&mut flags, 0x0F, 0x0F);
    assert_eq!(get_flag(flags, HALF_CARRY_FLAG), true);
    assert_eq!(get_flag(flags, CARRY_FLAG), false);

    flags = 0;
    update_carry_flags_add_u8(&mut flags, 0x0F, 0x01);
    assert_eq!(get_flag(flags, HALF_CARRY_FLAG), true);
    assert_eq!(get_flag(flags, CARRY_FLAG), false);

    flags = 0;
    update_carry_flags_add_u8(&mut flags, 0xFF, 0x01);
    assert_eq!(get_flag(flags, HALF_CARRY_FLAG), true);
    assert_eq!(get_flag(flags, CARRY_FLAG), true);

    flags = 0;
    update_carry_flags_add_u16(&mut flags, 0x0FFF, 0x0001);
    assert_eq!(get_flag(flags, HALF_CARRY_FLAG), true);
    assert_eq!(get_flag(flags, CARRY_FLAG), false);

    flags = 0;
    update_carry_flags_add_u16(&mut flags, 0xFFFF, 0x0001);
    assert_eq!(get_flag(flags, HALF_CARRY_FLAG), true);
    assert_eq!(get_flag(flags, CARRY_FLAG), true);
}
