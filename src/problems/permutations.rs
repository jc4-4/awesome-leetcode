// https://leetcode.com/problems/permutations/
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut cur = vec![-1; nums.len()];
    let mut used = vec![false; nums.len()];
    backtrack(0, &nums, &mut used, &mut cur, &mut res);
    res
}

fn backtrack(
    index: usize,
    nums: &Vec<i32>,
    used: &mut Vec<bool>,
    cur: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    // solution?
    if index == nums.len() {
        res.push(cur.clone());
        return;
    }

    // explore
    for i in 0..nums.len() {
        if used[i] {
            continue;
        }

        used[i] = true;
        cur[i] = nums[index];
        backtrack(index + 1, nums, used, cur, res);
        used[i] = false;
        cur[i] = -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permute_one() {
        assert_eq!(permute(vec![1]), vec![vec![1]]);
    }

    #[test]
    fn permute_two() {
        assert_eq!(permute(vec![1, 2]), vec![vec![1, 2], vec![2, 1]]);;
    }

    #[test]
    fn permute_three() {
        // assert_eq!(permute(vec![1, 2, 3]), vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]]);
        assert_eq!(permute(vec![1, 2, 3]), vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![3, 1, 2], vec![2, 3, 1], vec![3, 2, 1]]);
    }
}