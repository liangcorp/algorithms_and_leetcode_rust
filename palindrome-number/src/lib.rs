pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    } else if x == 0 {
        return true;
    }

    let length = ((x as f32).log10() + 1_f32) as i32;
    let mut num_vec: Vec<i32> = vec![];
    let mut x_num = x;

    for _ in 0..length {
        num_vec.push(x_num % 10);
        x_num /= 10;
    }

    for i in 0..length as usize {
        if num_vec[i] != num_vec[length as usize - 1 - i] {
            return false;
        }
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
