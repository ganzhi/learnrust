// https://leetcode.com/explore/interview/card/google/61/trees-and-graphs/3067/

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
      right: None
    }
  }
}

struct Solution {

}

// Paste below things below
use std::rc::Rc;
use std::cell::RefCell;

static mut max: i32 = -20000;

impl Solution {
    fn path_sum(root: &Rc<RefCell<TreeNode>>) -> i32 {
        let node = root.borrow();
        let mut sum = node.val;
        let mut single = node.val;
        let mut left_sum:i32 =0;
        let mut right_sum:i32 = 0;
        if let Some(ref left) = node.left {
            left_sum = Solution::path_sum(left);
            if left_sum > 0 {
                sum += left_sum;
                single = sum;
            }
        }
        if let Some(ref right) = node.right {
            right_sum = Solution::path_sum(right);
            if right_sum > 0 {
                sum += right_sum;
                if right_sum > left_sum {
                    single = node.val + right_sum;
                }
            }
        }
        unsafe{
            if sum > max {
                max = sum;
            }
        }
        return single;
    }

    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(ref node) = root {
            unsafe{
                max = -999999;
            }
            Solution::path_sum(node);
            unsafe {
                return max;
            }
        }
        5
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(-3))));
    let res = Solution::max_path_sum(root);
    println!("The result is {}!", res);
}
