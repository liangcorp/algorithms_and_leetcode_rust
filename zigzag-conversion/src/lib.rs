pub mod zigzag {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut new_s: Vec<char> = vec![];
        let old_s: Vec<char> = s.chars().collect();
        let mut counter: i32 = 0;
        let mut gap = num_rows + num_rows / 2;

        while gap > 0 {
            while (counter as usize) < s.len() {
                new_s.push(old_s[counter as usize]);
                counter += gap;
            }
            gap -= num_rows / 2;
            counter -= gap;
        }

        String::from_iter(new_s)
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn test_convert() {
            use super::*;

            assert_eq!("PAHNAPLSIIGYIR", convert(String::from("PAYPALISHIRING"), 3));
            assert_eq!("PAHNAPLSIIGYIR", convert(String::from("PINALSIGYAHRPI"), 4));
            assert_eq!("A", convert(String::from("A"), 1));
        }
    }
}
