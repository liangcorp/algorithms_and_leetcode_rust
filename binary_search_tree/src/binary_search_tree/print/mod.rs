/*
 * Name: Computer Algorithms
 * Author: Chen Liang
 * Description: Implementation of binary search tree
 * Date: 21-January-2024
 */

use crate::binary_search_tree::Node;

impl Node {
    pub fn inorder(&self) {
        if self.left.is_some() && self.right.is_some() {
            self.left.as_ref().unwrap().inorder();
            print!("{}  ", self.data);
            self.right.as_ref().unwrap().inorder();
        } else {
            print!("{}  ", self.data);
        }
    }

    pub fn preorder(&self) {
        if self.left.is_some() && self.right.is_some() {
            print!("{}  ", self.data);
            self.left.as_ref().unwrap().preorder();
            self.right.as_ref().unwrap().preorder();
        } else {
            print!("{}  ", self.data);
        }
    }

    pub fn postorder(&self) {
        if self.left.is_some() && self.right.is_some() {
            self.left.as_ref().unwrap().postorder();
            self.right.as_ref().unwrap().postorder();
            print!("{}  ", self.data);
        } else {
            print!("{}  ", self.data);
        }
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
