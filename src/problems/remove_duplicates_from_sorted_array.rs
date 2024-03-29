// https://leetcode.com/problems/remove-duplicates-from-sorted-array
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    // u points to the last unique element
    // first element is always unique if exist
    let mut u = 0;
    // [1, 2, 3]
    //  ^u ^i
    for i in 1..nums.len() {
        if nums[u] != nums[i] {
            u += 1;
            // technically, we can skip copy if u=i, but this actually slows the
            // submission by 2ms
            nums[u] = nums[i];
        }
    }
    1 + u as i32
}

#[cfg(test)]
mod tests {
    use crate::problems::remove_duplicates_from_sorted_array::remove_duplicates;

    #[test]
    fn test_distinct() {
        assert_eq!(remove_duplicates(&mut vec![1, 2, 3]), 3);
    }

    #[test]
    fn test_dup() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2);
    }

    #[test]
    fn test_all_dup() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 1]), 1);
    }

    #[test]
    fn test_all_but_one() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 1, 2]), 2);
    }

    #[test]
    fn test_dup_in_between() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2, 2, 3, 4, 4, 5, 5, 5, 5]), 5);
    }

    #[test]
    fn test_not_consecutive() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 21, 21, 31, 41, 41, 52, 52, 52, 52]), 5);
    }

    #[test]
    fn test_empty() {
        assert_eq!(remove_duplicates(&mut vec![]), 0);
    }
}
