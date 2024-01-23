/*
 * Name: Computer Algorithms
 * Author: Chen Liang
 * Description: Implementation of binary search tree
 * Date: 21-January-2024
 */

use std::cmp::Ordering;

use crate::binary_search_tree::Node;

impl Node {
    pub fn in_order(&self) {
        if self.left.is_some() && self.right.is_some() {
            if let Some(left) = self.left.as_ref() {
                left.in_order();
            }
            print!("{}  ", self.data);
            if let Some(right) = self.right.as_ref() {
                right.in_order();
            }
        } else {
            print!("{}  ", self.data);
        }
    }

    pub fn pre_order(&self) {
        if self.left.is_some() && self.right.is_some() {
            print!("{}  ", self.data);
            if let Some(left) = self.left.as_ref() {
                left.pre_order();
            }
            if let Some(right) = self.right.as_ref() {
                right.pre_order();
            }
        } else {
            print!("{}  ", self.data);
        }
    }

    pub fn post_order(&self) {
        if self.left.is_some() && self.right.is_some() {
            if let Some(left) = self.left.as_ref() {
                left.post_order();
            }
            if let Some(right) = self.right.as_ref() {
                right.post_order();
            }
            print!("{}  ", self.data);
        } else {
            print!("{}  ", self.data);
        }
    }

    pub fn given_level(&self, level: i32) {
        match level.cmp(&0) {
            Ordering::Equal => {
                print!("{}  ", self.data)
            }
            Ordering::Greater => {
                if let Some(left) = self.left.as_ref() {
                    left.given_level(level - 1);
                }
                if let Some(right) = self.right.as_ref() {
                    right.given_level(level - 1);
                }
            }
            Ordering::Less => (),
        }
    }

    pub fn leaf_nodes(&self) {
        if self.left.is_some() || self.right.is_some() {
            if let Some(left) = self.left.as_ref() {
                left.leaf_nodes();
            }
            if let Some(right) = self.right.as_ref() {
                right.leaf_nodes();
            }
        } else {
            print!("{}  ", self.data);
        }
    }

    pub fn none_leaf_nodes(&self) {
        if self.left.is_some() || self.right.is_some() {
            print!("{}  ", self.data);
            if let Some(left) = self.left.as_ref() {
                left.leaf_nodes();
            }
            if let Some(right) = self.right.as_ref() {
                right.leaf_nodes();
            }
        }
    }

    pub fn left_nodes(&self) {
        match self.left.as_ref() {
            Some(left) => {
                print!("{}  ", self.data);
                left.left_nodes();
            }
            None => {
                print!("{}", self.data);
                println!();
            }
        }
    }

    pub fn right_nodes(&self) {
        match self.right.as_ref() {
            Some(right) => {
                print!("{}  ", self.data);
                right.right_nodes();
            }
            None => {
                print!("{}", self.data);
                println!();
            }
        }
    }
}
