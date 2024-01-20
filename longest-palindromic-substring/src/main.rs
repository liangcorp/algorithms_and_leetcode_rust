mod longest_palindromic;

use longest_palindromic as lp;

fn main() {
    println!("{}", lp::center::find_string(String::from("babad")));
    println!("{}", lp::center::find_string(String::from("cbbd")));
    println!("{}", lp::center::find_string(String::from("a")));
    // println!("{}", lp::center::find_string(String::from("abb")));
    // println!("{}", lp::center::find_string(String::from("bb")));
    println!("{}", lp::center::find_string(String::from("aacabdkacaa")));
    println!("{}", lp::brute::find_string(String::from("aacabdkacaa")));
}
