fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let middle;
    let mut middles = vec![];
    let mut final_vec: Vec<i32> = vec![];

    let mut nums1_counter = 0;
    let mut nums2_counter = 0;

    let mut is_end_of_num1 = false;
    let mut is_end_of_num2 = false;

    if nums1.is_empty() {
        is_end_of_num1 = true;
    } else if nums2.is_empty() {
        is_end_of_num2 = true;
    }
    // Uneven vector walk.
    // Move element of a vector 1 counter further
    // The element is choicen between elements from two vectors
    // Break the loop if reach the end of one vector
    while !is_end_of_num1 && !is_end_of_num2 {
        if nums2[nums2_counter] <= nums1[nums1_counter] {
            final_vec.push(nums2[nums2_counter]);
            nums2_counter += 1;
            if nums2_counter == nums2.len() {
                is_end_of_num2 = true;
            }
        } else if nums2[nums2_counter] > nums1[nums1_counter] {
            final_vec.push(nums1[nums1_counter]);
            nums1_counter += 1;
            if nums1_counter == nums1.len() {
                is_end_of_num1 = true;
            }
        }
    }

    // Append the rest of the longer vector to the final_vec
    if is_end_of_num2 {
        final_vec.extend_from_slice(&nums1[nums1_counter..]);
    } else if is_end_of_num1 {
        final_vec.extend_from_slice(&nums2[nums2_counter..]);
    }

    // Identify final_vec has odd or even number of elements
    if (nums1.len() + nums2.len()) % 2 == 0 {
        // Even number of elements
        // Find the locations of the medium elements
        middles.push((nums1.len() + nums2.len()) / 2 - 1);
        middles.push((nums1.len() + nums2.len()) / 2);

        // Return the average
        (final_vec[middles[0]] as f64 + final_vec[middles[1]] as f64) / 2.0
    } else {
        // Odd number of elements
        // Find the location of the medium element
        middle = (nums1.len() + nums2.len()) / 2;

        // Return the average
        final_vec[middle] as f64
    }
}

fn main() {
    println!("1, {:?}", find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
    println!("2, {:?}", find_median_sorted_arrays(vec![1, 3], vec![2]));
    println!("3, {:?}", find_median_sorted_arrays(vec![1], vec![2]));
    println!(
        "4, {:?}",
        find_median_sorted_arrays(vec![1, 1, 1, 1, 2], vec![1, 3])
    );
    println!(
        "5, {:?}",
        find_median_sorted_arrays(vec![1, 1, 1, 1, 1, 1], vec![2, 2])
    );
}
