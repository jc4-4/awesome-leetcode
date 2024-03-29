// https://leetcode.com/problems/subsets/
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    backtrack(0, &nums, &mut vec![], &mut res);
    res
}

// generate all subsets from nums[i..n)
fn backtrack(i: usize, nums: &Vec<i32>, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    res.push(cur.clone());
    for j in i..nums.len() {
        // nums[j] is chosen
        cur.push(nums[j]);
        backtrack(j + 1, nums, cur, res);
        cur.pop();
    }
}

// generate all subsets from nums[0..i)
fn recurse(i: usize, nums: &Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if i == 0 {
        return;
    }

    // generate all subset in nums[0..i-1)
    recurse(i - 1, nums, res);

    // for each subset, add nums[i] to it
    for mut subset in res.clone() {
        subset.push(nums[i - 1]);
        res.push(subset);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(subsets(vec![]), vec![vec![]]);
    }

    #[test]
    fn test_one() {
        assert_eq!(subsets(vec![1]), vec![vec![], vec![1]]);
    }

    #[test]
    fn test_two() {
        assert_eq!(
            subsets(vec![1, 2]),
            vec![vec![], vec![1], vec![1, 2], vec![2]]
        );
    }

    #[test]
    fn test_three() {
        assert_eq!(
            subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3],
            ]
        );
    }
}
