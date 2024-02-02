pub fn is_match(s: String, p: String) -> bool {
    let mut p_vec: Vec<char> = vec![];

    for (i, c) in p.chars().enumerate() {
        match &c {
            '.' => {
                if i == 0 {
                    if let Some(t) = s.chars().next() {
                        p_vec.push(t);
                    }
                } else if let Some(t) = p.chars().nth(i - 1) {
                    p_vec.push(t);
                }
            }
            '*' => {
                let mut s_vec: Vec<char> = s.as_str()[i..].to_string().chars().collect();
                p_vec.append(&mut s_vec);
            }
            _ => p_vec.push(c),
        }
    }

    s == String::from_iter(p_vec)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_match_1() {
        assert!(!is_match(String::from("aa"), String::from("a")));
    }

    #[test]
    fn test_is_match_2() {
        assert!(is_match(String::from("aa"), String::from("a*")));
    }

    #[test]
    fn test_is_match_3() {
        assert!(is_match(String::from("aa"), String::from(".*")));
    }

    #[test]
    fn test_is_match_4() {
        assert!(is_match(String::from("aab"), String::from("c*a*b")));
    }
}
