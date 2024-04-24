// Example
// [7, 1, 5, 3, 6, 4]
//     ^--------^
// [4,3,2,1]
// descending => no profit
// [1,2,3,4]
// ascending => max profit at 0->n
// [1,3,2] (^ shape)
//  ^-^  (upward bit)
// [2,1,3] (v shape)
//    ^-^ (upward bit)
// find ascending subsequence
// [2, 4, 1, x]
//        ^ Q: do you give up 2 and take 1?
//          yes, because if `x` ever turns a profit, then x-1 > x-2

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut buy = None;
    for i in 0..prices.len() {
        let price = prices[i];
        match buy {
            Some(p) if price >= p => {
                profit = profit.max(price - p);
            }
            _ => buy = Some(price),
        }
    }
    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn test_descending() {
        assert_eq!(max_profit(vec![4, 3, 2, 1]), 0);
    }

    #[test]
    fn test_ascending() {
        assert_eq!(max_profit(vec![1, 2, 3, 4]), 3);
    }

    #[test]
    fn test_updown() {
        assert_eq!(max_profit(vec![1, 3, 2]), 2);
    }

    #[test]
    fn test_downup() {
        assert_eq!(max_profit(vec![2, 1, 3]), 2);
    }
}
