pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut lcp: Vec<char> = strs.first().unwrap().clone().chars().collect();
    let mut flush_pos: usize = strs.first().unwrap().len();
    let mut tmp: char;

    for s in strs.iter().skip(1) {
        for (j, c) in lcp.iter().enumerate() {

            if let Some(t) = s.chars().nth(j) {
                tmp = t;
            } else {
                flush_pos = j;
                break;
            };

            if *c != tmp {
                flush_pos = j;
                break;
            }
        }

        lcp.drain(flush_pos..);
    }

    String::from_iter(lcp.iter())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_2common() {
        assert_eq!(longest_common_prefix(vec![String::from("flower"), String::from("flow"), String::from("flight")]), "fl");
    }

    #[test]
    fn test_nocommon() {
        assert_eq!(longest_common_prefix(vec![String::from("dog"), String::from("racecar"), String::from("car")]), "");
    }

    #[test]
    fn test_fullcommon() {
        assert_eq!(longest_common_prefix(vec![String::from("flower"), String::from("flower"), String::from("flower"), String::from("flower")]), "flower");
    }
}
