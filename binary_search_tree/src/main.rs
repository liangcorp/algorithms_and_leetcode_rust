/*
 * Name: Computer Algorithms
 * Author: Chen Liang
 * Description: Implementation of binary search tree
 * Date: 21-January-2024
 */

mod binary_search_tree;

fn main() {
    let mut tree_node = binary_search_tree::Node::new(50);
    tree_node.insert(30);
    tree_node.insert(20);
    tree_node.insert(40);
    tree_node.insert(70);
    tree_node.insert(60);
    tree_node.insert(80);

    println!("Inorder: ");
    tree_node.inorder();
    println!();

    println!("Preorder: ");
    tree_node.preorder();
    println!();

    println!("Postorder: ");
    tree_node.postorder();
    println!();

    println!("Leaf Nodes:");
    tree_node.leaf_nodes();
    println!();

    println!("None Leaf Nodes:");
    tree_node.none_leaf_nodes();
    println!();
}
