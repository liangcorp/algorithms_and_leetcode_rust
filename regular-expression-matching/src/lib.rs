pub fn is_match(s: String, p: String) -> bool {
    if !p.contains('.') && !p.contains('*') && s.len() != p.len() {
        false
    } else if p.contains('.') && !p.contains('*') {
        if s.is_empty() && p.is_empty() {
            return true;
        }

        if s.chars().next() == p.chars().next() || p.starts_with('.') {
            is_match(s.as_str()[1..].to_string(), p.as_str()[1..].to_string());
        }

        true
    } else {
        if s.is_empty() && p.is_empty() {
            return true;
        }
        if let Some(c) = p.chars().nth(1) {
            if (c == '*' && s.chars().next() == p.chars().next())
                || (c == '*' && p.starts_with('.'))
            {
                return true;
            } else {
                is_match(s.as_str()[1..].to_string(), p.as_str()[2..].to_string());
            }
        }
        false
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
}
