/*
 * Name: Computer Algorithms
 * Author: Chen Liang
 * Description: Implementation of binary search tree
 * Date: 21-January-2024
 */

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use crate::binary_search_tree::Node;

impl Node {
    pub fn insert(&mut self, data: i32) {
        let new_node;

        match data.cmp(&self.data) {
            Ordering::Less => match &self.left {
                Some(left) => {
                    (*left).borrow_mut().insert(data);
                }
                None => {
                    new_node = Rc::new(RefCell::new(Node::new(data)));
                    self.left = Some(new_node);
                }
            },
            Ordering::Greater => match &self.right {
                Some(right) => {
                    (*right).borrow_mut().insert(data);
                }
                None => {
                    new_node = Rc::new(RefCell::new(Node::new(data)));
                    self.right = Some(new_node);
                }
            },
            Ordering::Equal => (),
        }
    }
}
