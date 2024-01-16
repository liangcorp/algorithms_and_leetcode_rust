use std::collections::HashMap;

/// Loop and go through elements of the given vector
///     If found a number (x) that add up to "target" exists in the HashMap
///         Return the index of the elemnt (in the vector)
///         and x (stored in the vector)
///
///     Else add x into Hashmap (with it's index in the vector)
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut h_map = HashMap::new();
    let mut x: i32;

    for (i, y) in nums.iter().enumerate() {
        x = target - y;

        if let Some(&j) = h_map.get(&x) {
            return vec![j as i32, i as i32];
        }

        h_map.insert(y, i);
    }

    vec![0, 0]      // Dummy return
}

fn main() {
    let nums = vec![3, 2, 4];
    println!("Result: {:?}", two_sum(nums, 6));
}
