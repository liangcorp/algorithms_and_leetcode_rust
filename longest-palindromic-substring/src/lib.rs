pub mod longest_palindromic;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::longest_palindromic::brute::find_string as brute_fs;
    use super::longest_palindromic::center::find_string as center_fs;

    #[test]
    fn test_find_string_via_center() {
        assert_eq!(brute_fs(String::from("aacabdkacaa")), center_fs(String::from("aacabdkacaa")));
    }

    #[test]
    fn test_find_string_via_center_2() {
        assert_eq!(brute_fs(String::from("babad")), center_fs(String::from("babad")));
    }
}
