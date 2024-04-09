// https://leetcode.com/problems/add-binary

pub fn add_binary(a: String, b: String) -> String {
    let (mut a, mut b) = (&a, &b);
    if a.len() < b.len() {
        std::mem::swap(&mut b, &mut a);
    }
    assert!(a.len() >= b.len());

    // a:     1010
    // b:       10
    // left:  1010
    // right: 0010
    let left = a.clone();
    let mut right = b.clone();
    right.insert_str(0, &"0".repeat(a.len() - b.len()));

    let mut c = Vec::with_capacity(a.len() + 1);
    let mut carry = '0';
    for i in (0..a.len()).rev() {
        // Note: it would be simpler to convert digits to u32
        match (
            carry,
            left.chars().nth(i).unwrap(),
            right.chars().nth(i).unwrap(),
        ) {
            ('0', '0', '0') => {
                c.push('0');
            }
            ('0', '0', '1') => {
                c.push('1');
            }
            ('0', '1', '0') => {
                c.push('1');
            }
            ('0', '1', '1') => {
                c.push('0');
                carry = '1';
            }
            ('1', '0', '0') => {
                c.push('1');
                carry = '0';
            }
            ('1', '0', '1') => {
                c.push('0');
            }
            ('1', '1', '0') => {
                c.push('0');
            }
            ('1', '1', '1') => {
                c.push('1');
            }
            _ => {}
        }
    }

    if carry == '1' {
        c.push('1');
    }

    // reverse c
    c.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!("0", add_binary("0".to_string(), "0".to_string()));
    }

    #[test]
    fn test_single_digits() {
        assert_eq!("0", add_binary("0".to_string(), "0".to_string()));
        assert_eq!("1", add_binary("1".to_string(), "0".to_string()));
        assert_eq!("1", add_binary("0".to_string(), "1".to_string()));
        assert_eq!("10", add_binary("1".to_string(), "1".to_string()));
    }

    #[test]
    fn test_samples() {
        assert_eq!("100", add_binary("11".to_string(), "1".to_string()));
        assert_eq!("10101", add_binary("1010".to_string(), "1011".to_string()));
    }
}
