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
}
