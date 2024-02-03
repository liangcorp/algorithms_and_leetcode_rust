pub fn is_match(s: String, p: String) -> bool {
    // let mut p_vec: Vec<char> = vec![];

    // let mut s_i = 0;
    // let mut p_i = 0;

    if s.chars().next() == p.chars().next() || p.starts_with('*') {
        is_match(s.as_str()[1..].to_string(), p.as_str()[1..].to_string());
    } else if p.starts_with('*') {
        is_match(
            s.as_str()[2..].to_string(),
            p.as_str()[2..].to_string(),
        );
    } else {
        return false;
    }

    // s == String::from_iter(p_vec)
    true
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
