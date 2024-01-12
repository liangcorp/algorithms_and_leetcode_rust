use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut h_map = HashMap::new();

    for (i, y) in nums.iter().enumerate() {
        let x = target - y;

        if let Some(&j) = h_map.get(&x) {
            return vec![j as i32, i as i32];
        }

        h_map.insert(y, i);
    }

    // let mut v_map: Vec<i32> = Vec::new();
    //
    // for (i, y) in nums.iter().enumerate() {
    //     let x = target - y;
    //
    //     if let Some(j) = v_map.iter().position(|k| *k == x) {
    //         return vec![j as i32, i as i32];
    //     }
    //     v_map.push(*y)
    // }

    vec![0, 0]
}

fn main() {
    let nums = vec![3, 2, 4];
    println!("Result: {:?}", two_sum(nums, 6));
}
