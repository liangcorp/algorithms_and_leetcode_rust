/*
 * Name: Computer Algorithms
 * Author: Chen Liang
 * Description: Implementation of binary search tree
 * Date: 21-January-2024
 */

use crate::Node;

use std::cmp::Ordering;

impl Node {
    pub fn in_order(&self) {
        if let Some(left) = self.left.as_ref() {
            left.in_order();
        }
        print!("{}  ", self.data);
        if let Some(right) = self.right.as_ref() {
            right.in_order();
        }
    }

    pub fn pre_order(&self) {
        print!("{}  ", self.data);
        if let Some(left) = self.left.as_ref() {
            left.pre_order();
        }
        if let Some(right) = self.right.as_ref() {
            right.pre_order();
        }
    }

    pub fn post_order(&self) {
        if let Some(left) = self.left.as_ref() {
            left.post_order();
        }
        if let Some(right) = self.right.as_ref() {
            right.post_order();
        }
        print!("{}  ", self.data);
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
        if self.left.is_none() && self.right.is_none() {
            print!("{}  ", self.data);
        } else {
            if let Some(left) = self.left.as_ref() {
                left.leaf_nodes();
            }
            if let Some(right) = self.right.as_ref() {
                right.leaf_nodes();
            }
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

    pub fn only_left(&self) {
        match self.left.as_ref() {
            Some(left) => {
                print!("{}  ", self.data);
                left.only_left();
            }
            None => {
                print!("{}", self.data);
                println!();
            }
        }
    }

    pub fn only_right(&self) {
        match self.right.as_ref() {
            Some(right) => {
                print!("{}  ", self.data);
                right.only_right();
            }
            None => {
                print!("{}", self.data);
                println!();
            }
        }
    }
}
