#[allow(dead_code)]
fn longest_common_subsequence(a: &str, b: &str) -> String {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    let (len_a, len_b) = (a.len(), b.len()); // store the lengths

    // solution[i][j] is he lengths of the LCS
    // between a[0..i-1] and b[0..j-1]
    //
    let mut solution = vec![vec![0; len_b + 1]; len_a + 1];

    for (i, mi) in a.iter().enumerate() {
        for (j, mj) in b.iter().enumerate() {
            // if mi = mj, there is a new common char
            // otherwise, take the best of the two solutions
            // at (i-1, j) and (i, j-1)

            solution[i + 1][j + 1] = if mi == mj {
                solution[i][j] + 1
            } else {
                solution[i][j + 1].max(solution[i + 1][j])
            }
        }
    }

    // reconsitute the solution from the lengths
    let mut result: Vec<char> = Vec::new();
    let (mut i, mut j) = (len_a, len_b);

    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            result.push(a[i - 1]);
            i -= 1;
            j -= 1;
        } else if solution[i - 1][j] > solution[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    result.reverse();
    result.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs() {
        // emtpy test case
        assert_eq!(longest_common_subsequence("", ""), "");
        assert_eq!(longest_common_subsequence("", "acbd"), "");
        assert_eq!(longest_common_subsequence("asdf", ""), "");

        assert_eq!(longest_common_subsequence("abcd", "a"), "a");
        assert_eq!(longest_common_subsequence("adbc", "b"), "b");
        assert_eq!(longest_common_subsequence("qwerty", "rty"), "rty");

        assert_eq!(longest_common_subsequence("abcdgh", "aedfhr"), "adh");
        assert_eq!(longest_common_subsequence("aggtab", "gtxayb"), "gtab");
    }
}
