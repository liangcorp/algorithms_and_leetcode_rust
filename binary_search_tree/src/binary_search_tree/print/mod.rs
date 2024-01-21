/*
 * Name: Computer Algorithms
 * Author: Chen Liang
 * Description: Implementation of binary search tree
 * Date: 21-January-2024
 */

use crate::binary_search_tree::Node;

impl Node {
    pub fn in_order(&self) {
        if self.left.is_some() && self.right.is_some() {
            self.left.as_ref().unwrap().in_order();
            print!("{}  ", self.data);
            self.right.as_ref().unwrap().in_order();
        } else {
            print!("{}  ", self.data);
        }
    }

    pub fn pre_order(&self) {
        if self.left.is_some() && self.right.is_some() {
            print!("{}  ", self.data);
            self.left.as_ref().unwrap().pre_order();
            self.right.as_ref().unwrap().pre_order();
        } else {
            print!("{}  ", self.data);
        }
    }

    pub fn post_order(&self) {
        if self.left.is_some() && self.right.is_some() {
            self.left.as_ref().unwrap().post_order();
            self.right.as_ref().unwrap().post_order();
            print!("{}  ", self.data);
        } else {
            print!("{}  ", self.data);
        }
    }

    pub fn level_order(&self) {

    }

    pub fn leaf_nodes(&self) {
        if self.left.is_some() || self.right.is_some() {
            self.left.as_ref().unwrap().leaf_nodes();
            self.right.as_ref().unwrap().leaf_nodes();
        } else {
            print!("{}  ", self.data);
        }
    }

    pub fn none_leaf_nodes(&self) {
        if self.left.is_some() || self.right.is_some() {
            print!("{}  ", self.data);
            self.left.as_ref().unwrap().none_leaf_nodes();
            self.right.as_ref().unwrap().none_leaf_nodes();
        }
    }
}
