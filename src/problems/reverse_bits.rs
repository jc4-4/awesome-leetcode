pub fn reverse_bits(x: u32) -> u32 {
    let mut r = 0;
    for i in 0..32 {
        let mask = 1 << i;
        if x & mask == mask {
            r += 1 << (31 - i);
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_one_bit() {
        assert_eq!(
            reverse_bits(0b00000000000000000000000000000001u32),
            0b10000000000000000000000000000000u32
        );
    }

    #[test]
    fn test_reverse() {
        assert_eq!(
            reverse_bits(0b00000010100101000001111010011100u32),
            0b00111001011110000010100101000000u32
        );
    }
}
