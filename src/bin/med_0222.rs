













// 222. Count Complete Tree Nodes
// Binary Tree 11


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
use std::rc::Rc;
use std::cell::RefCell;
struct Solution {}
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = std::collections::VecDeque::new();
        if root.is_none() { return 0;}
        let mut sum = 0;
        stack.push_back(root);
        while !stack.is_empty() {
            let temp = stack.pop_front().unwrap();
            if let Some(node) = temp {
                stack.push_back(node.borrow().left.clone());
                stack.push_back(node.borrow().right.clone());
                sum += 1;
            }
        }
        sum
    }
}

fn main() {
}
