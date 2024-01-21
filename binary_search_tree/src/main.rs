/*
 * Name: Computer Algorithms
 * Author: Chen Liang
 * Description: Implementation of binary search tree
 * Date: 21-January-2024
 */


mod binary_search_tree;
// use computer_algorithms::binary_search_tree;

fn main() {
    let mut tree_node = binary_search_tree::Node::new(8);
    tree_node.insert(3);
    tree_node.insert(1);
    tree_node.insert(6);
    tree_node.insert(10);
    tree_node.insert(14);
    tree_node.insert(13);
    tree_node.insert(4);
    tree_node.insert(7);
    tree_node.inorder();
}
