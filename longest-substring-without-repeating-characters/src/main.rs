use std::{char, collections::HashMap};

///  Check each character of the string's existence in the HashMap
///
///  If a duplicate character was found
///      Flush the HashMap from the existing duplicate
///      to the beginning
///
///      Insert the character into HashMap
///
///  keeping checking the length of the HashMap
///  and find the max
fn length_of_longest_substring(s: String) -> i32 {
    let mut max: i32 = 0;
    let mut char_map: HashMap<char, i32> = HashMap::new();

    for (i, c) in s.chars().enumerate() {
        match char_map.get(&c) {
            Some(&z) => {
                char_map.retain(|_, &mut k| k > z);
                char_map.insert(c, i as i32);
            }
            None => {
                char_map.insert(c, i as i32);
            }
        }

        if max < char_map.len() as i32 {
            max = char_map.len() as i32;
        }
    }
    max
}

fn main() {
    println!(
        "bbbbbb: {}",
        length_of_longest_substring(String::from("bbbbbb"))
    );
    println!(
        "abcabcbb: {}",
        length_of_longest_substring(String::from("abcabcbb"))
    );
    println!(
        "pwwkew: {}",
        length_of_longest_substring(String::from("pwwkew"))
    );
    println!("\" \": {}", length_of_longest_substring(String::from(" ")));
    println!(
        "dvdf: {}",
        length_of_longest_substring(String::from("dvdf"))
    );
    println!("au: {}", length_of_longest_substring(String::from("au")));
    println!(
        "bbtablud: {}",
        length_of_longest_substring(String::from("bbtablud"))
    );
    println!(
        "ohvhjdml: {}",
        length_of_longest_substring(String::from("ohvhjdml"))
    );
}
