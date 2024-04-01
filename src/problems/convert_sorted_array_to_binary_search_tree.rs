// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree

use std::{cell::RefCell, rc::Rc};

use crate::shared::*;

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    return slice_to_tree(&nums);

    fn slice_to_tree(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let mid = nums.len() / 2;

        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[mid],
            left: slice_to_tree(&nums[..mid]),
            right: slice_to_tree(&nums[mid + 1..]),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl From<TreeNode> for Option<Rc<RefCell<TreeNode>>> {
        fn from(node: TreeNode) -> Self {
            Some(Rc::new(RefCell::new(node)))
        }
    }

    fn new_node(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    #[test]
    fn test_example() {
        assert_eq!(
            sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
            TreeNode {
                left: TreeNode {
                    left: new_node(-10).into(),
                    ..new_node(-3)
                }
                .into(),
                right: TreeNode {
                    left: new_node(5).into(),
                    ..new_node(9)
                }
                .into(),
                ..new_node(0)
            }
            .into()
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            sorted_array_to_bst(vec![1, 3]),
            TreeNode {
                left: new_node(1).into(),
                ..new_node(3)
            }
            .into()
        );
    }
}
