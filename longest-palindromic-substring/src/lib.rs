pub mod longest_palindromic;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::longest_palindromic::brute::find_string as brute_fs;
    use super::longest_palindromic::center::find_string as center_fs;
    use super::longest_palindromic::manacher::find_string as manacher_fs;

    #[test]
    fn test_find_string_via_brute_force() {
        assert_eq!("aca", brute_fs(String::from("aacabdkacaa")));
        assert_eq!("bab", brute_fs(String::from("babad")));
        assert_eq!("bb", brute_fs(String::from("cbbd")));
        assert_eq!("aaaaa", brute_fs(String::from("aaaaa")));
        assert_eq!("aaaa", brute_fs(String::from("aaaa")));
    }

    #[test]
    fn test_find_string_via_center() {
        assert_eq!("aca", center_fs(String::from("aacabdkacaa")));
        assert_eq!("bab", center_fs(String::from("babad")));
        assert_eq!("bb", center_fs(String::from("cbbd")));
        assert_eq!("aaaaa", center_fs(String::from("aaaaa")));
        assert_eq!("aaaa", center_fs(String::from("aaaa")));
    }

    #[test]
    fn test_find_string_via_manacher() {
        assert_eq!("aca", manacher_fs(String::from("aacabdkacaa")));
        assert_eq!("bab", manacher_fs(String::from("babad")));
        assert_eq!("bb", manacher_fs(String::from("cbbd")));
        assert_eq!("aaaaa", manacher_fs(String::from("aaaaa")));
        assert_eq!("aaaa", manacher_fs(String::from("aaaa")));
    }
}
