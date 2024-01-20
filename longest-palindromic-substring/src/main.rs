#[allow(dead_code)]
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

#[allow(dead_code)]
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

fn longest_palindrome_center(s: String) -> String {
    let mut new_s: Vec<char> = Vec::new();
    for c in s.chars() {
        new_s.push('#');
        new_s.push(c);
    }
    new_s.push('#');

    let mut palindrom_radii = vec![0; new_s.len()];
    let mut center = 0;
    let mut radius = 0;

    // if s.len() == 1 {
    //     return s;
    // }

    while center < new_s.len() {
        radius = 0;

        while center + radius < new_s.len() && new_s[center - radius] == new_s[center + radius] {
            radius += 1;

            if radius > center {
                break;
            }
        }

        palindrom_radii[center] = radius;
        center += 1;
    }

    center = *palindrom_radii.iter().max().unwrap();
    println!("{:?}, center {}", palindrom_radii, center);
    // println!(
    //     " result {}, char {}",
    //     center,
    //     &s.as_str()[center - radius - 1..center + radius]
    // );
    s.as_str()[center / 2 - radius / 2..center / 2 + radius / 2].to_string()
}

fn main() {
    println!("{}", longest_palindrome_center(String::from("babad")));
    println!("{}", longest_palindrome_center(String::from("cbbd")));
    println!("{}", longest_palindrome_center(String::from("a")));
    // println!("{}", longest_palindrome_center(String::from("abb")));
    // println!("{}", longest_palindrome_center(String::from("bb")));
    println!("{}", longest_palindrome_center(String::from("aacabdkacaa")));
}
