use std::collections::VecDeque;

// https://leetcode.com/problems/palindrome-number/
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    // push digits into a deque
    let mut y = x;
    let mut deq = VecDeque::with_capacity(10);
    while y > 0 {
        deq.push_front(y % 10);
        y /= 10;
    }

    // check mismatch in the first and last digits
    while deq.len() >= 2 {
        if deq.pop_front().unwrap()!= deq.pop_back().unwrap() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn test_case2() {
        assert!(!is_palindrome(-121));
    }

    #[test]
    fn test_case3() {
        assert!(!is_palindrome(10));
    }
}
