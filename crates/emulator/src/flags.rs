/// **Z** - Zero flag
pub const FLAG_ZERO: u8 = 0b1000_0000;
/// **N** - Subtraction flag (BCD)
pub const FLAG_SUBTRACT: u8 = 0b0100_0000;
/// **H** - Half Carry flag (BCD)
pub const FLAG_HALF_CARRY: u8 = 0b0010_0000;
/// **C** - Carry flag
pub const FLAG_CARRY: u8 = 0b0001_0000;

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
        FLAG_HALF_CARRY,
        ((a & 0x0F) + (b & 0x0F)) & (1 << 4) != 0,
    );
    set_flag(flags, FLAG_CARRY, (a as u16 + b as u16) & (1 << 8) != 0);
}

#[inline(always)]
pub fn update_carry_flags_add_u16(flags: &mut u8, a: u16, b: u16) {
    set_flag(
        flags,
        FLAG_HALF_CARRY,
        ((a & 0x0FFF) + (b & 0x0FFF)) & (1 << 12) != 0,
    );
    set_flag(flags, FLAG_CARRY, (a as u32 + b as u32) & (1 << 16) != 0);
}

#[test]
fn test_carry_update() {
    let mut flags: u8 = 0;
    update_carry_flags_add_u8(&mut flags, 0x00, 0x00);
    assert_eq!(get_flag(flags, FLAG_HALF_CARRY), false);
    assert_eq!(get_flag(flags, FLAG_CARRY), false);

    flags = 0;
    update_carry_flags_add_u8(&mut flags, 0x01, 0x02);
    assert_eq!(get_flag(flags, FLAG_HALF_CARRY), false);
    assert_eq!(get_flag(flags, FLAG_CARRY), false);

    flags = 0;
    update_carry_flags_add_u8(&mut flags, 0x0F, 0x0F);
    assert_eq!(get_flag(flags, FLAG_HALF_CARRY), true);
    assert_eq!(get_flag(flags, FLAG_CARRY), false);

    flags = 0;
    update_carry_flags_add_u8(&mut flags, 0x0F, 0x01);
    assert_eq!(get_flag(flags, FLAG_HALF_CARRY), true);
    assert_eq!(get_flag(flags, FLAG_CARRY), false);

    flags = 0;
    update_carry_flags_add_u8(&mut flags, 0xFF, 0x01);
    assert_eq!(get_flag(flags, FLAG_HALF_CARRY), true);
    assert_eq!(get_flag(flags, FLAG_CARRY), true);

    flags = 0;
    update_carry_flags_add_u16(&mut flags, 0x0FFF, 0x0001);
    assert_eq!(get_flag(flags, FLAG_HALF_CARRY), true);
    assert_eq!(get_flag(flags, FLAG_CARRY), false);

    flags = 0;
    update_carry_flags_add_u16(&mut flags, 0xFFFF, 0x0001);
    assert_eq!(get_flag(flags, FLAG_HALF_CARRY), true);
    assert_eq!(get_flag(flags, FLAG_CARRY), true);
}

#[test]
fn test_flag_setters_getters() {
    let mut flag: u8 = 0x00;

    assert_eq!(get_flag(flag, FLAG_ZERO), false);
    assert_eq!(get_flag(flag, FLAG_SUBTRACT), false);
    assert_eq!(get_flag(flag, FLAG_HALF_CARRY), false);
    assert_eq!(get_flag(flag, FLAG_CARRY), false);

    set_flag(&mut flag, FLAG_ZERO, true);
    assert_eq!(flag, FLAG_ZERO);
    assert_eq!(get_flag(flag, FLAG_ZERO), true);

    set_flag(&mut flag, FLAG_SUBTRACT, true);
    assert_eq!(flag, FLAG_ZERO | FLAG_SUBTRACT);
    assert_eq!(get_flag(flag, FLAG_SUBTRACT), true);

    set_flag(&mut flag, FLAG_HALF_CARRY, true);
    assert_eq!(flag, FLAG_ZERO | FLAG_SUBTRACT | FLAG_HALF_CARRY);
    assert_eq!(get_flag(flag, FLAG_HALF_CARRY), true);

    set_flag(&mut flag, FLAG_CARRY, true);
    assert_eq!(
        flag,
        FLAG_ZERO | FLAG_SUBTRACT | FLAG_HALF_CARRY | FLAG_CARRY
    );
    assert_eq!(get_flag(flag, FLAG_CARRY), true);
}
