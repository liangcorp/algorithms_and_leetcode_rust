fn is_palindrome(s: &str) -> bool {
    let str_vec: Vec<char> = s.chars().collect();

    for x in 0..str_vec.len() {
        if str_vec[x] != str_vec[str_vec.len() - x - 1] {
            return false;
        }
    }
    true
}

fn longest_palindrome(s: String) -> String {
    let mut max = 0;
    let mut result_str = String::new();
    let mut y;

    for x in 0..s.len() {
        y = x;
        while y < s.len() + 1 {
            if is_palindrome(&s.as_str()[x..y]) && max < y - x {
                max = y - x;
                result_str = s.as_str()[x..y].to_string();
            }
            y += 1;
        }
    }

    result_str
}

fn main() {
    // println!("{}", longest_palindrome(String::from("babad")));
    // println!("{}", longest_palindrome(String::from("cbbd")));
    // println!("{}", longest_palindrome(String::from("a")));
    // println!("{}", longest_palindrome(String::from("abb")));
    // println!("{}", longest_palindrome(String::from("bb")));
    println!("{}", longest_palindrome(String::from("aacabdkacaa")));
}
