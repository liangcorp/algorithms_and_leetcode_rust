mod delete;
mod print;
mod search;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct Node {
    data: i32,
    left: Pointer,
    right: Pointer,
}

type Pointer = Option<Box<Node>>;

impl Node {
    pub fn new(data: i32) -> Node {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, data: i32) {
        let new_node;

        match data.cmp(&self.data) {
            Ordering::Less => {
                if self.left.is_some() {
                    if let Some(left) = self.left.as_mut() {
                        left.insert(data);
                    }
                } else {
                    new_node = Box::new(Node::new(data));
                    self.left = Some(new_node);
                }
            }
            Ordering::Greater => {
                if self.right.is_some() {
                    if let Some(right) = self.right.as_mut() {
                        right.insert(data);
                    }
                } else {
                    new_node = Box::new(Node::new(data));
                    self.right = Some(new_node);
                }
            }
            Ordering::Equal => (),
        }
    }

    pub fn tree_height(&self) -> i32 {
        let l_depth = match self.left.as_ref() {
            Some(left) => left.tree_height(),
            None => 0,
        };

        let r_depth = match self.right.as_ref() {
            Some(right) => right.tree_height(),
            None => 0,
        };

        if l_depth > r_depth {
            l_depth + 1
        } else {
            r_depth + 1
        }
    }

    pub fn node_count(&self) -> i32 {
        let l_node_count = match self.left.as_ref() {
            Some(left) => left.node_count(),
            None => 0,
        };

        let r_node_count = match self.right.as_ref() {
            Some(right) => right.node_count(),
            None => 0,
        };

        l_node_count + r_node_count + 1
    }
}
