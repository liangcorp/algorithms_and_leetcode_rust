use crate::binary_search_tree::Node;

impl Node {
    pub fn min_value(&self) -> i32 {
        match self.left.as_ref() {
            Some(left) => left.min_value(),
            None => self.data,
        }
    }
}
