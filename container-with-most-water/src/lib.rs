#[allow(dead_code)]
fn max_area(height: Vec<i32>) -> i32 {
    let mut max: i32 = 0;
    let mut chosen_h: i32;

    for (i, h) in height.iter().enumerate() {
        for (j, other_h) in height.iter().enumerate().skip(i) {
            if h < other_h {
                chosen_h = *h;
            } else {
                chosen_h = *other_h;
            }
            if max < (j as i32 - i as i32) * chosen_h {
                max = (j as i32 - i as i32) * chosen_h;
            }
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
