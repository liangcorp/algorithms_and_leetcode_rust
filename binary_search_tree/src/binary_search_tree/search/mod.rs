use crate::binary_search_tree::Node;

impl Node {
    pub fn min_value(&self) -> i32 {
        match &self.left {
            Some(left) => (*left).borrow().min_value(),
            None => self.data,
        }
    }
}
