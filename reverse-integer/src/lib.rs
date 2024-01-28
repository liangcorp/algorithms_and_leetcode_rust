#[allow(dead_code)]
fn reverse(x: i32) -> i32 {
    let x_str = x.to_string();
    let mut is_minus = false;
    let mut result_str: String;

    let x_vec: Vec<char> = x_str.chars().collect();
    if x_vec[0] == '-' {
        is_minus = true;
    }
    result_str = x_vec.iter().rev().collect();

    if is_minus {
        result_str.pop();
        format!("{}{}", '-', result_str).parse().unwrap_or(0)
    } else {
        result_str.parse().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_integer_123() {
        assert_eq!(321, reverse(123));
    }

    #[test]
    fn test_reverse_integer_minus_123() {
        assert_eq!(-321, reverse(-123));
    }

    #[test]
    fn test_reverse_integer_minus_120() {
        assert_eq!(21, reverse(120));
    }
}
#[test]
fn test_reverse_integer_minus_1534236469() {
    assert_eq!(21, reverse(1534236469));
}
