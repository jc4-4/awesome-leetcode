// https://leetcode.com/problems/valid-parentheses/
pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                match stack.pop() {
                    Some(d) if is_pair(d, c) => (),
                    _ => return false,
                }
                // let d = stack.pop();
                // if d.is_none() || !is_pair(d.unwrap(), c) {
                //     return false;
                // }
            },
            _ => panic!("Unexpected char: {}", c),
        }
    }

    return stack.is_empty();
}

fn is_pair(c: char, d: char) -> bool {
    match (c, d) {
        ('(', ')') | ('[', ']') | ('{', '}') => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(is_valid("()".to_string()), true);
    }

    #[test]
    fn test_case2() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn test_case3() {
        assert_eq!(is_valid("(]".to_string()), false);
    }

    #[test]
    fn test_nested() {
        assert_eq!(is_valid("((()))[]{}".to_string()), true);
    }

    #[test]
    fn test_empty() {
        assert_eq!(is_valid("(".to_string()), false);
    }
}
