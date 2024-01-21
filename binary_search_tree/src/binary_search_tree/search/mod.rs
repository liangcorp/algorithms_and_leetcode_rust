use crate::binary_search_tree::Node;

impl Node {
    pub fn min_value(&self) -> i32 {
        if self.left.is_some() {
            self.left.as_ref().unwrap().min_value()
        } else {
            self.data
        }
    }
}
