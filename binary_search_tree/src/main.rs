/*
 * Name: Computer Algorithms
 * Author: Chen Liang
 * Description: Implementation of binary search tree
 * Date: 21-January-2024
 */

fn main() {
    let mut tree_node = binary_search_tree::Node::new(50);

    tree_node.insert(30);
    tree_node.insert(20);
    tree_node.insert(40);
    tree_node.insert(70);
    tree_node.insert(60);
    tree_node.insert(80);

    // println!("{:?}", tree);

    println!("Inorder: ");
    tree_node.in_order();
    println!();

    println!("Preorder: ");
    tree_node.pre_order();
    println!();

    println!("Postorder: ");
    tree_node.post_order();
    println!();

    println!("Leaf Nodes:");
    tree_node.leaf_nodes();
    println!();

    println!("None Leaf Nodes:");
    tree_node.none_leaf_nodes();
    println!();

    println!("Number of levels: {}", tree_node.tree_height());
    println!("Number of nodes: {}", tree_node.node_count());

    println!("Print at level");
    for i in 0..tree_node.tree_height() {
        tree_node.given_level(i);
        println!();
    }

    println!("Smallest value: {}", tree_node.min_value());

    println!("Print only right nodes");
    tree_node.only_right();

    println!("Print only left nodes");
    tree_node.only_left();

    println!("Delete");
    tree_node.delete(80);
    tree_node.in_order();
    println!();
}
