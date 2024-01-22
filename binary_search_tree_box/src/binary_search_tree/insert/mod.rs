/*
 * Name: Computer Algorithms
 * Author: Chen Liang
 * Description: Implementation of binary search tree
 * Date: 21-January-2024
 */

use std::cmp::Ordering;

use crate::binary_search_tree::Node;

impl Node {
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
}
