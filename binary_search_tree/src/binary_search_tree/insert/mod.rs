use std::cmp::Ordering;

use crate::binary_search_tree::Node;

impl Node {
    pub fn insert(&mut self, data: i32) {
        let new_node;

        match data.cmp(&self.data) {
            Ordering::Less => {
                if self.left.is_some() {
                    self.left.as_mut().unwrap().insert(data);
                } else {
                    new_node = Box::new(Node::new(data));
                    self.left = Some(new_node);
                }
            }
            Ordering::Greater => {
                if self.right.is_some() {
                    self.right.as_mut().unwrap().insert(data);
                } else {
                    new_node = Box::new(Node::new(data));
                    self.right = Some(new_node);
                }
            }
            Ordering::Equal => (),
        }
    }
}
