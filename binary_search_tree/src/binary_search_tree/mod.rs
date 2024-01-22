mod insert;
mod print;
mod search;
mod delete;

pub struct Node {
    data: i32,
    left: Pointer,
    right: Pointer,
}

type Pointer = Option<Box<Node>>;

impl Node {
    pub fn new(data: i32) -> Node {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    pub fn tree_height(&self) -> i32 {
        let l_depth;
        let r_depth;
        if self.left.is_some() || self.right.is_some() {
            match self.left.as_ref() {
                Some(left) => l_depth = left.tree_height(),
                None => l_depth = 1,
            }

            match self.right.as_ref() {
                Some(right) => r_depth = right.tree_height(),
                None => r_depth = 1,
            }

            if l_depth > r_depth {
                l_depth + 1
            } else {
                r_depth + 1
            }
        } else {
            1
        }
    }

    pub fn node_count(&self) -> i32 {
        let l_node_count;
        let r_node_count;

        if self.left.is_some() || self.right.is_some() {
            match self.left.as_ref() {
                Some(left) => l_node_count = left.node_count(),
                None => l_node_count = 0,
            }
            match self.right.as_ref() {
                Some(right) => r_node_count = right.node_count(),
                None => r_node_count = 0,
            }
            l_node_count + r_node_count + 1

        } else {
            1
        }
    }
}
