pub fn is_match(s: String, p: String) -> bool {
    if p.is_empty() {
        return s.is_empty();
    }

    let is_first_match =
        !s.is_empty() && (s.chars().next() == p.chars().next() || p.starts_with('.'));

    if p.len() >= 2 && p.chars().nth(1).unwrap() == '*' {
        return is_match(s.clone(), p.as_str()[2..].to_string())
            || (is_first_match && is_match(s.as_str()[1..].to_string(), p));
    } else {
        return is_first_match
            && is_match(s.as_str()[1..].to_string(), p.as_str()[1..].to_string());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

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
