// https://leetcode.com/problems/remove-element

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    while i < nums.len() {
        if nums[i] == val {
            nums.swap_remove(i);
        } else {
            i += 1;
        }
    }
    i as i32
}

#[cfg(test)]
mod tests {
    use googletest::{
        assert_that,
        matchers::{elements_are, eq},
    };

    use super::*;

    #[test]
    fn test_example() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(2, remove_element(&mut nums, 3));
        assert_that!(nums, elements_are![eq(2), eq(2)]);
    }

    #[test]
    fn test_example2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(5, remove_element(&mut nums, 2));
        assert_that!(nums, elements_are![eq(0), eq(1), eq(4), eq(0), eq(3),]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(0, remove_element(&mut vec![], 1));
    }

    #[test]
    fn test_one_unmatch() {
        assert_eq!(1, remove_element(&mut vec![1], 2));
    }

    #[test]
    fn test_one_match() {
        let mut nums = vec![1];
        assert_eq!(0, remove_element(&mut nums, 1));
    }

    #[test]
    fn test_all_match() {
        let mut nums = vec![1, 1, 1];
        assert_eq!(0, remove_element(&mut nums, 1));
    }
}
