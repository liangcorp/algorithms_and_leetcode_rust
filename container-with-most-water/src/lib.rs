#[allow(dead_code)]
fn max_area(height: Vec<i32>) -> i32 {
    let mut max: i32 = 0;
    let mut l_edge: i32 = height[0];
    let mut l_pos: usize = 0;
    let mut r_edge: i32 = height[height.len() - 1];
    let mut r_pos: usize = height.len() - 1;

    while l_pos != r_pos {
        if l_edge < r_edge {
            if max < l_edge * (r_pos - l_pos) as i32 {
                max = l_edge * (r_pos - l_pos) as i32;
            }
            l_edge = height[l_pos + 1];
            l_pos += 1;
        } else {
            if max < r_edge * (r_pos - l_pos) as i32 {
                max = r_edge * (r_pos - l_pos) as i32;
            }
            r_edge = height[r_pos - 1];
            r_pos -= 1;
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
