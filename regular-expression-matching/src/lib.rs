pub mod brute;
pub mod cache;
pub mod vector;

#[cfg(test)]
mod tests {
    // use crate::brute;
    use crate::cache::is_match;

    #[test]
    fn test_is_match_1() {
        assert!(!is_match(String::from("aa"), String::from("a")));
    }

    #[test]
    fn test_is_match_dot_only() {
        assert!(is_match(String::from("ab"), String::from("a.")));
    }

    #[test]
    fn test_is_match_dot_only_2() {
        assert!(is_match(String::from("abc"), String::from("a.c")));
    }
    #[test]
    fn test_is_match_star_only() {
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

    #[test]
    fn test_is_match_mississippi() {
        assert!(!is_match(
            String::from("mississippi"),
            String::from("mis*is*p*.")
        ));
    }

    #[test]
    fn test_is_match_aaaaaaaaaaaaaaaaaaab() {
        assert!(!is_match(
            String::from("aaaaaaaaaaaaaaaaaaab"),
            String::from("a*a*a*a*a*a*a*a*a*a*")
        ));
    }
}
