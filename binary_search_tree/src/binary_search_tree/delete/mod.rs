/*
 * Name: Computer Algorithms
 * Author: Chen Liang
 * Description: Implementation of binary search tree
 * Date: 21-January-2024
 */

use crate::binary_search_tree::Node;

use std::cmp::Ordering;

impl Node {
    pub fn delete(&mut self, data: i32) {
        match data.cmp(&self.data) {
            Ordering::Less => {
                if let Some(left) = self.left.as_mut() {
                    left.delete(data);
                }
            }
            Ordering::Greater => {
                if let Some(right) = self.right.as_mut() {
                    right.delete(data);
                }
            }
            Ordering::Equal => {
                if self.left.is_none() && self.right.is_none() {
                    let _ = self;
                } else if self.left.is_some() && self.right.is_none() {
                    self::Node::new(self.left.as_mut().unwrap().data);
                } else if self.left.is_none() && self.right.is_some() {
                    self::Node::new(self.right.as_mut().unwrap().data);
                }
            }
        }
    }
}
