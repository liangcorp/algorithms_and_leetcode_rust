pub mod zigzag {
    use std::usize;

    // Break the strings into even chucks starting with
    // num_rows + num_rows - 2
    // loop through the string chucks
    // Push the first element into new_s
    // Push the head and tail of the rest of the chucks
    // into the new_s
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut new_s: Vec<char> = vec![];
        let mut slice_start = 0;
        let mut slice_end = num_rows + num_rows - 2;
        let distance = num_rows + num_rows - 2;
        let mut string_chucks: Vec<&str> = vec![];

        if s.len() == 1 {
            return s;
        }

        // divide the string into chucks of size = num_rows + num_rows - 2
        while (slice_end as usize) < s.len() {
            string_chucks.push(&s[slice_start as usize..slice_end as usize]);
            slice_start = slice_end;
            slice_end += distance;
        }
        string_chucks.push(&s[slice_start as usize..]);

        for i in 0..num_rows {
            for (j, s_chuck) in string_chucks.iter().enumerate() {
                if i == 0 {
                    // add top characters of the string chucks to the vector
                    new_s.push(s_chuck.chars().next().unwrap());
                } else if i == num_rows - 1 || i as usize == s_chuck.len() - 1 {
                    // if reach center of string chuck
                    // or the length of string chuck is less than given row number
                    if let Some(c) = s_chuck.chars().nth(i as usize) {
                        new_s.push(c);
                    }
                } else if i as usize > s_chuck.len() - 1 {
                    // do nothing if the row number is larger than the string lentgh
                } else if i != num_rows - 1 {
                    // add the characters at the beginning and the end to the vector
                    if let Some(c) = s_chuck.chars().nth(i as usize) {
                        new_s.push(c);
                    }

                    if let Some(c) = s_chuck.chars().nth(s_chuck.len() - i as usize) {
                        new_s.push(c);
                    }
                }
            }
        }
        String::from_iter(new_s)
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn test_convert_3_row() {
            use super::*;

            assert_eq!("PAHNAPLSIIGYIR", convert(String::from("PAYPALISHIRING"), 3));
        }

        #[test]
        fn test_convert_4_row() {
            use super::*;

            assert_eq!("PINALSIGYAHRPI", convert(String::from("PAYPALISHIRING"), 4));
        }

        #[test]
        fn test_convert_single_char() {
            use super::*;

            assert_eq!("A", convert(String::from("A"), 1));
        }
    }
}
