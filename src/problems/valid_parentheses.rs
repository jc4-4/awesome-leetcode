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

    stack.is_empty()
}

fn is_pair(c: char, d: char) -> bool {
    matches!((c, d), ('(', ')') | ('[', ']') | ('{', '}'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert!(is_valid("()".to_string()));
    }

    #[test]
    fn test_case2() {
        assert!(is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_case3() {
        assert!(!is_valid("(]".to_string()));
    }

    #[test]
    fn test_nested() {
        assert!(is_valid("((()))[]{}".to_string()));
    }

    #[test]
    fn test_empty() {
        assert!(!is_valid("(".to_string()));
    }
}
