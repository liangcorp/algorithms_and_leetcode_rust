mod longest_palindromic;

use std::time::SystemTime;

use longest_palindromic::brute::find_string as brute;
use longest_palindromic::center::find_string as center;

fn main() {
    // println!("{}", center(String::from("babad")));
    // println!("{}", center(String::from("cbbd")));
    // println!("{}", center(String::from("a")));
    // println!("{}", center(String::from("abb")));
    // println!("{}", center(String::from("bb")));

    let now = SystemTime::now();
    println!("{}", center(String::from("aacabdkacaa")));

    match now.elapsed() {
        Ok(elapsed) => println!("Center method in nano seconds: {}", elapsed.as_nanos()),
        Err(e) => println!("Error: {e:?}"),
    }

    let now = SystemTime::now();
    println!("{}", brute(String::from("aacabdkacaa")));

    match now.elapsed() {
        Ok(elapsed) => println!("Brute force method in nano seconds: {}", elapsed.as_nanos()),
        Err(e) => println!("Error: {e:?}"),
    }
}
