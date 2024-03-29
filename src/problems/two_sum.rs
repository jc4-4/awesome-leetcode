use std::collections::HashMap;

// https://leetcode.com/problems/two-sum/
// simple one-pass solution
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());

    for i in 0..nums.len() {
        let comp = target - nums[i];
        match map.get(&comp) {
            Some(&y) => return vec![y, i as i32],
            None => map.insert(nums[i], i as i32),
        };
    }
    panic!("no solution found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_case2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_case3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
