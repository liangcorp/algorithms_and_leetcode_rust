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
