/// Converts two u8s into a u16
///
/// # Arguments
///
/// * `msb` - Most significant byte of the resulting u16
/// * `lsb` - Least significant byte of the resulting u16
///
/// # Examples
///
/// ```
/// use util::u8_to_u16;
/// let result = u8_to_u16(0xAB, 0xCD);
///
/// assert_eq!(0xABCD, result);
/// ```
pub fn u8_to_u16(msb: u8, lsb: u8) -> u16 {
    ((msb as u16) << 8) | lsb as u16
}

/// Splits a u16 into a tuple of two u8s with order: most_significant_byte, least_significant_byte
///
/// # Arguments
///
/// * `val` - A u16 to be split.
///
/// # Examples
///
/// ```
/// use util::u16_to_u8;
/// let (msb, lsb) = u16_to_u8(0xABCD);
///
/// assert_eq!(0xAB, msb);
/// assert_eq!(0xCD, lsb);
/// ```
pub fn u16_to_u8(val: u16) -> (u8, u8) {
    let msb = (val >> 8) as u8;
    let lsb = (val & 0xFF) as u8;
    (msb, lsb)
}

/// Returns a boolean indicating whether a half-carry will occur during the addition of a and b.
pub fn half_carried_add16(a: u16, b: u16) -> bool {
    let a = a & 0x0FFF;
    let b = b & 0x0FFF;
    (a.wrapping_add(b) & 0x1000) == 0x1000
}

/// Returns a boolean indicating whether a half-carry will occur during the addition of a and b.
pub fn half_carried_add8(a: u8, b: u8) -> bool {
    // https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/
    let a = a & 0xF;
    let b = b & 0xF;
    (a.wrapping_add(b) & 0x10) == 0x10
}

/// Returns a boolean indicating whether a half-carry will occur during the subtraction of b from a.
pub fn half_carried_sub8(a: u8, b: u8) -> bool {
    // https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/
    let a = a & 0xF;
    let b = b & 0xF;
    (a.wrapping_sub(b) & 0x10) == 0x10
}

pub fn bit(x: u8, bit: u8) -> u8 {
    x & (1 << bit)
}

pub fn set_bits(original: u8, new: u8, mask: u8) -> u8 {
    (original & !mask) | (new & mask)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_to_u16() {
        assert_eq!(0x0000, u8_to_u16(0x00, 0x00));
        assert_eq!(0xFFFF, u8_to_u16(0xFF, 0xFF));
        assert_eq!(0x00FF, u8_to_u16(0x00, 0xFF));
        assert_eq!(0xFF00, u8_to_u16(0xFF, 0x00));
        assert_eq!(0x4A2F, u8_to_u16(0x4A, 0x2F));
    }

    #[test]
    fn test_u16_to_u8() {
        assert_eq!((0x00, 0x00), u16_to_u8(0x0000));
        assert_eq!((0xFF, 0xFF), u16_to_u8(0xFFFF));
        assert_eq!((0x00, 0xFF), u16_to_u8(0x00FF));
        assert_eq!((0xFF, 0x00), u16_to_u8(0xFF00));
        assert_eq!((0x4A, 0x2F), u16_to_u8(0x4A2F));
    }

    #[test]
    fn test_half_carried_add8() {
        assert_eq!(false, half_carried_add8(0b00000000, 0b00000000));
        assert_eq!(false, half_carried_add8(0b00000000, 0b00000001));
        assert_eq!(false, half_carried_add8(0b00000001, 0b00000000));
        assert_eq!(true, half_carried_add8(0b00001010, 0b00001100));
        assert_eq!(true, half_carried_add8(0b00000110, 0b00001100));
    }

    #[test]
    fn test_half_carried_sub8() {
        assert_eq!(false, half_carried_sub8(0b00000000, 0b00000000));
        assert_eq!(false, half_carried_sub8(0b00000001, 0b00000000));
        assert_eq!(true, half_carried_sub8(0b00000000, 0b00000001));
        assert_eq!(true, half_carried_sub8(0b00000000, 0b00001000));
        assert_eq!(true, half_carried_sub8(0b00000110, 0b00001100));
    }
}
