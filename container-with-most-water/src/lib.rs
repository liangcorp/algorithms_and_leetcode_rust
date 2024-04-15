#[allow(dead_code)]
fn max_area(height: Vec<i32>) -> i32 {
    let mut max: i32 = 0;
    let mut used_height;

    for (i, element) in height.iter().enumerate() {
        if *element > height[height.len() - i - 1] {
            used_height = height[height.len() - i - 1];
        } else {
            used_height = *element;
        }

        let temp = used_height * (height.len() - i - 1) as i32;

        if max < temp {
            max = temp;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_container() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test_container2() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}
