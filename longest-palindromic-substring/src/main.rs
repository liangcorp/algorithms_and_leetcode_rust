fn is_str_palindrome(s: &str) -> bool {
    // Simple check for panidrome
    // Loop from end to center
    // Check charaters from end to center
    for i in 0..s.len() {
        if s.as_bytes()[i] != s.as_bytes()[s.len() - i - 1] {
            return false;
        }
    }

    true
}

fn longest_palindrome(s: String) -> String {
    let mut slice_start;
    let mut slice_end;

    for i in 0..s.len() {
        // Slice the strings from the longest to the shortest
        slice_start = 0;
        slice_end = s.len() - i;

        while slice_end != s.len() + 1 {
            if is_str_palindrome(&s.as_str()[slice_start..slice_end]) {
                return s.as_str()[slice_start..slice_end].to_string();
            }
            slice_start += 1;
            slice_end += 1;
        }
    }
    String::from("fail")
}

fn main() {
    println!("{}", longest_palindrome(String::from("babad")));
    println!("{}", longest_palindrome(String::from("cbbd")));
    println!("{}", longest_palindrome(String::from("a")));
    println!("{}", longest_palindrome(String::from("abb")));
    println!("{}", longest_palindrome(String::from("bb")));
    println!("{}", longest_palindrome(String::from("aacabdkacaa")));
}
