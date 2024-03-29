// https://leetcode.com/problems/generate-parentheses

use std::collections::HashSet;

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];
    backtrack(n, &mut String::new(), &mut HashSet::new(), &mut res);
    res
}

fn backtrack(length: i32, current: &mut String, visited: &mut HashSet<String>, result: &mut Vec<String>) {
    // found a solution, save it!
    if current.len() as i32 == 2 * length {
        result.push(current.clone());
        return;
    }

    // insert a parenthesis at each position.
    for i in 0..=current.len() {
        current.insert_str(i, "()");
        // prune searching if duplicate
        // example: ^() = ()^ = ()()
        if !visited.contains(current) {
            backtrack(length, current, visited, result);
            visited.insert(current.clone());
        }
        current.drain(i..i + 2);
    }
}

#[cfg(test)]
mod tests {
    use googletest::{
        assert_that,
        matchers::{eq, unordered_elements_are},
    };

    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(generate_parenthesis(1), ["()"]);
    }

    #[test]
    fn test_two() {
        assert_that!(
            generate_parenthesis(2),
            unordered_elements_are![eq("(())"), eq("()()")]
        );
    }

    #[test]
    fn test_three() {
        assert_that!(
            generate_parenthesis(3),
            unordered_elements_are![
                eq("((()))"),
                eq("(())()"),
                eq("()()()"),
                eq("(()())"),
                eq("()(())")
            ]
        );
    }
}
