// https://leetcode.com/problems/longest-common-prefix/description/
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let len = strs.len();
    assert!(len > 0);
    if len == 1 {
        return strs[0].clone();
    }

    assert!(len > 1);
    for i in 0..strs[0].len() {
        // invariant: strs[0][..i] is a common prefix
        // check if c is the i-th char of remaining strings
        let c = strs[0].chars().nth(i);
        for j in 1..strs.len() {
            match strs[j].chars().nth(i) {
                Some(x) if Some(x) == c => {},
                _ => return strs[0][..i].to_string(),
            }
        }
    }
    strs[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_string(vec: Vec<&str>) -> Vec<String> {
        vec.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_case1() {
        assert_eq!(
            longest_common_prefix(str_to_string(vec!["flower", "flow", "flight"])),
            "fl".to_string()
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            longest_common_prefix(str_to_string(vec!["dog", "racecar", "car"])),
            "".to_string()
        );
    }

    #[test]
    fn test_overflow() {
        assert_eq!(
            longest_common_prefix(str_to_string(vec!["flower", "flow"])),
            "flow".to_string()
        );
    }
}
