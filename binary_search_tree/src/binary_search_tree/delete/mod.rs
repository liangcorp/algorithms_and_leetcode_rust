/*
 * Name: Computer Algorithms
 * Author: Chen Liang
 * Description: Implementation of binary search tree
 * Date: 21-January-2024
 */

use crate::binary_search_tree::Node;

use std::{cmp::Ordering, borrow::BorrowMut};

impl Node {
#[allow(dead_code)]
    pub fn delete(&mut self, data: i32) {
        match data.cmp(&self.data) {
            Ordering::Less => {
                if let Some(left) = &self.left {
                    (**left).borrow_mut().delete(data);
                }
            }
            Ordering::Greater => {
                if let Some(right) = &self.right {
                    (**right).borrow_mut().delete(data);
                }
            }

            Ordering::Equal => {
                if self.left.is_none() && self.right.is_none() {
                        drop(self.borrow_mut());
                }

                // if let Some(left) = self.left {
                //     self = (*left).take();
                // }
                //
                // if let Some(right) = self.right {
                //     self = (*right).clone();
                // }
            }
        }
    }
}
