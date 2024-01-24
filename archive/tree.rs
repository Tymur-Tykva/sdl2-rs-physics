use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TreeNode {
    // contains: Vec<_>,
    children: Vec<TreeNodeRef>,
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;
