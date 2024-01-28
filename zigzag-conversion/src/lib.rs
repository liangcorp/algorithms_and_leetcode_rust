pub mod zigzag;

#[cfg(test)]
mod tests {
    use crate::zigzag::calculate::convert;

    #[test]
    fn test_convert_3_row() {
        assert_eq!("PAHNAPLSIIGYIR", convert(String::from("PAYPALISHIRING"), 3));
    }

    #[test]
    fn test_convert_4_row() {
        assert_eq!("PINALSIGYAHRPI", convert(String::from("PAYPALISHIRING"), 4));
    }

    #[test]
    fn test_convert_abc() {
        assert_eq!("ABC", convert(String::from("ABC"), 3));
    }
    #[test]

    fn test_convert_two_char() {
        assert_eq!("AB", convert(String::from("AB"), 1));
    }

    #[test]
    fn test_convert_single_char() {
        assert_eq!("A", convert(String::from("A"), 1));
    }
}
