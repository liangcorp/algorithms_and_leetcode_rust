use std::{cell::RefCell, rc::Rc};

mod delete;
mod insert;
mod print;
mod search;

#[derive(Debug)]
pub struct Node {
    data: i32,
    left: Pointer,
    right: Pointer,
}

type Pointer = Option<Rc<RefCell<Node>>>;

impl Node {
    pub fn new(data: i32) -> Node {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    pub fn tree_height(&self) -> i32 {
        if self.left.is_none() || self.right.is_none() {
            return 1;
        }

        let l_depth = match &self.left {
            Some(left) => left.borrow().tree_height(),
            None => 1,
        };

        let r_depth = match &self.right {
            Some(right) => right.borrow().tree_height(),
            None => 1,
        };

        if l_depth > r_depth {
            l_depth + 1
        } else {
            r_depth + 1
        }
    }

    pub fn node_count(&self) -> i32 {
        if self.left.is_none() || self.right.is_none() {
            return 1;
        }

        let l_count = match &self.left {
            Some(left) => left.borrow().node_count(),
            None => 0,
        };

        let r_count = match &self.right {
            Some(right) => right.borrow().node_count(),
            None => 0,
        };

        l_count + r_count + 1
    }
}
