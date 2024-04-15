fn max_area(height: Vec<i32>) -> i32 {
    1
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_container() {
        assert_eq!(max_area(vec![]), 3);
    }

    #[test]
    fn test_container2() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(max_area(vec![]), 3);
    }
}
