// https://leetcode.com/problems/same-tree

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::{cell::RefCell, rc::Rc};
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(_), None) => false,
        (None, Some(_)) => false,
        (Some(x), Some(y)) => {
            let a = x.borrow();
            let b = y.borrow();
            a.val == b.val
                && is_same_tree(a.left.clone(), b.left.clone())
                && is_same_tree(a.right.clone(), b.right.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Reconstruct a binary tree from in-order traversal
    // For example, [4, 2, 5, 1, 6, 3, 7] becomes the following tree:
    //      1
    //    /   \
    //   2     3
    //  / \   / \
    // 4   5 6   7
    fn to_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }

        let mid = values.len() / 2;
        values[mid].map(|value| {
            Rc::new(RefCell::new(TreeNode {
                val: value,
                left: to_tree(&values[..mid]),
                right: to_tree(&values[mid + 1..]),
            }))
        })
    }

    #[test]
    fn test_same() {
        //    1
        //  2   3
        assert!(is_same_tree(
            to_tree(&[Some(2), Some(1), Some(3)]),
            to_tree(&[Some(2), Some(1), Some(3)])
        ))
    }

    #[test]
    fn test_diff_value() {
        assert!(!is_same_tree(
            to_tree(&[Some(2), Some(1), Some(3)]),
            to_tree(&[Some(3), Some(1), Some(2)])
        ))
    }

    #[test]
    fn test_empty_left() {
        assert!(!is_same_tree(
            to_tree(&[None, Some(1), Some(3)]),
            to_tree(&[Some(2), Some(1), Some(3)])
        ));
        assert!(!is_same_tree(
            to_tree(&[Some(2), Some(1), Some(3)]),
            to_tree(&[None, Some(1), Some(3)])
        ));
        assert!(is_same_tree(
            to_tree(&[None, Some(1), Some(3)]),
            to_tree(&[None, Some(1), Some(3)])
        ));
    }

    #[test]
    fn test_empty_right() {
        assert!(!is_same_tree(
            to_tree(&[Some(2), Some(1), None]),
            to_tree(&[Some(2), Some(1), Some(3)])
        ));
        assert!(!is_same_tree(
            to_tree(&[Some(2), Some(1), Some(3)]),
            to_tree(&[Some(2), Some(1), None])
        ));
        assert!(is_same_tree(
            to_tree(&[Some(2), Some(1), None]),
            to_tree(&[Some(2), Some(1), None])
        ));
    }

    #[test]
    fn test_deeper_tree() {
        //      1
        //   2    3
        // o  4  5  o
        assert!(is_same_tree(
            to_tree(&[None, Some(2), Some(4), Some(1), Some(5), Some(3), None]),
            to_tree(&[None, Some(2), Some(4), Some(1), Some(5), Some(3), None])
        ));

        assert!(!is_same_tree(
            to_tree(&[None, Some(2), Some(4), Some(1), None, Some(3), Some(5)]),
            to_tree(&[None, Some(2), Some(4), Some(1), Some(5), Some(3), None])
        ));

        assert!(!is_same_tree(
            to_tree(&[Some(4), Some(2), None, Some(1), Some(5), Some(3), None]),
            to_tree(&[None, Some(2), Some(4), Some(1), Some(5), Some(3), None])
        ));
    }
}
