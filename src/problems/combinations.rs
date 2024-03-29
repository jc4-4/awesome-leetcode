// https://leetcode.com/problems/combinations
fn combine1(n: i32, k: i32) -> Vec<Vec<i32>> {
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
    let mut x = combine1(n - 1, k - 1);
    for s in &mut x {
        s.push(n);
    }
    let mut y = combine1(n - 1, k);
    y.append(&mut x);
    y
}

// iterative solution below
fn combine2(n: i32, k: i32) -> Vec<Vec<i32>> {
    if k == 0 {
        return vec![vec![]];
    }

    // starts with [1..k]
    let mut result = vec![];
    let mut current: Vec<i32> = (1..=k).collect();
    result.push(current.clone());
    if n == k {
        return result;
    }

    let len = k as usize;
    let upper_bound = |j| n - k + 1 + j as i32;
    let mut i = len - 1;
    loop {
        if current[i] < upper_bound(i) {
            current[i] += 1;
            result.push(current.clone());
            continue;
        }

        while i > 0 && current[i] == upper_bound(i) {
            i -= 1;
        }

        if current[i] < upper_bound(i) {
            current[i] += 1;
            for m in i + 1..len {
                current[m] = current[i] + (m - i) as i32;
            }
            result.push(current.clone());
            i = len - 1;
        } else {
            assert!(i == 0 && current[i] == upper_bound(i));
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_three() {
        assert_eq!(combine2(3, 3), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn test_two_zero() {
        assert_eq!(combine2(2, 0), vec![vec![]]);
    }

    #[test]
    fn test_three_two() {
        assert_eq!(combine2(3, 2), vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
    }

    #[test]
    fn test_four_one() {
        assert_eq!(combine2(4, 1), vec![vec![1], vec![2], vec![3], vec![4]]);
    }

    #[test]
    fn test_four_two() {
        //   assert_eq!(combine2(4, 2), vec![vec![1, 2], vec![1, 3], vec![2, 3], vec![1, 4], vec![2, 4], vec![3, 4]]);
        assert_eq!(
            combine2(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        );
    }

    #[test]
    fn test_four_three() {
        assert_eq!(
            combine2(4, 3),
            vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 3, 4], vec![2, 3, 4]]
        );
    }

    #[test]
    fn test_four_four() {
        assert_eq!(combine2(4, 4), vec![vec![1, 2, 3, 4]]);
    }
}
