pub fn is_match(s: String, p: String) -> bool {

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
}
