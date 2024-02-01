pub fn is_palindrome(x: i32) -> bool {
    match x.cmp(&0) {
        std::cmp::Ordering::Equal => return true,
        std::cmp::Ordering::Less => return false,
        std::cmp::Ordering::Greater => (),
    }

    let length = ((x as f32).log10() + 1_f32) as i32;
    let mut x_num = x;
    let mut x_num2 = x;

    for i in 0..length {
        if x_num % 10 != x_num2 / 10_i32.pow((length - i - 1) as u32) {
            return false;
        }

        x_num /= 10;

        x_num2 -= x_num2 / 10_i32.pow((length - i - 1) as u32) * 10_i32.pow((length - i - 1) as u32);

    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_121() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn test_is_palindrome_negative_121() {
        assert!(!is_palindrome(-121));
    }

    #[test]
    fn test_is_palindrome_120() {
        assert!(!is_palindrome(120));
    }

    #[test]
    fn test_is_palindrome_100() {
        assert!(!is_palindrome(100));
    }

    #[test]
    fn test_is_palindrome_1221() {
        assert!(is_palindrome(1221));
    }
}
