// https://leetcode.com/problems/generate-parentheses

use std::collections::HashSet;

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];
    // backtrack(n, &mut String::new(), &mut HashSet::new(), &mut res);
    backtrack2(n, n, &mut String::with_capacity(2 * n as usize), &mut res);
    res
}

fn backtrack(
    length: i32,
    current: &mut String,
    visited: &mut HashSet<String>,
    result: &mut Vec<String>,
) {
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

fn backtrack2(num_open: i32, num_closed: i32, current: &mut String, result: &mut Vec<String>) {
    // a well-formed parentheses if every open ( has a closed ).
    if num_open == 0 && num_closed == 0 {
        result.push(current.clone());
        return;
    }

    // instead of inserting () in the middle, always place at the end.
    // try to place an open (
    if num_open > 0 {
        current.push('(');
        backtrack2(num_open - 1, num_closed, current, result);
        current.pop();
    }

    // to make sure this is well-formed, only place a closing ) if there is a
    // matching open (. This avoids cases like )(.
    if num_closed > num_open {
        current.push(')');
        backtrack2(num_open, num_closed - 1, current, result);
        current.pop();
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
