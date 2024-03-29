// https://leetcode.com/problems/combinations
fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    assert!(k <= n);

    // two base cases: empty set or a set of [1..n]
    if k == 0 {
        return vec![vec![]];
    }
    if k == n {
        return vec![(1..=n).collect()];
    }
    assert!(k < n);

    // recursive case: x * n + y, where
    // x = n chosen => add n to each set in x
    // y = n not chosen => no op
    let mut x = combine(n - 1, k - 1);
    for s in &mut x {
        s.push(n);
    }
    let mut y = combine(n - 1, k);
    y.append(&mut x);
    y
}

#[cfg(test)]
mod tests {
    use super::combine;

    #[test]
    fn test_three_three() {
        assert_eq!(combine(3, 3), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn test_two_zero() {
        assert_eq!(combine(2, 0), vec![vec![]]);
    }

    #[test]
    fn test_three_two() {
      assert_eq!(combine(3, 2), vec![vec![1,2], vec![1, 3], vec![2, 3]]);
    }

    #[test]
    fn test_four_two() {
      assert_eq!(combine(4, 2), vec![vec![1,2], vec![1, 3], vec![2, 3], vec![1, 4], vec![2, 4], vec![3, 4]]);
    }

}