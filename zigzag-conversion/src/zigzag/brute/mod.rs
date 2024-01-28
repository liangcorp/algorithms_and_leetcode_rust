// Break the strings into even chucks starting with
// num_rows + num_rows - 2
// loop through the string chucks
// Push the first element into new_s
// Push the head and tail of the rest of the chucks
// into the new_s
pub fn convert(s: String, num_rows: i32) -> String {
    let mut new_s = String::new();
    let mut slice_start = 0;
    let mut slice_end = num_rows + num_rows - 2;
    let distance = num_rows + num_rows - 2;
    let mut str_chucks: Vec<&str> = vec![];

    if num_rows == 1 {
        return s;
    }

    // divide the string into chucks of size = num_rows + num_rows - 2
    while (slice_end as usize) < s.len() {
        str_chucks.push(&s[slice_start as usize..slice_end as usize]);
        slice_start = slice_end;
        slice_end += distance;
    }

    // File the shorter string with '#' to make it even
    let filer_str = "#".repeat(distance as usize - s[slice_start as usize..].len());
    let temp = format!("{}{}", &s[slice_start as usize..], &filer_str);
    str_chucks.push(&temp);

    for i in 0..num_rows {
        for s_chuck in str_chucks.iter() {
            if i == 0 {
                // add top characters of the string chucks to
                // the vector
                if let Some(c) = s_chuck.chars().next() {
                    new_s.push(c);
                }
            } else if i == num_rows - 1 {
                // if reach center of string chuck
                // or the length of string chuck is less than
                // given row number
                if let Some(c) = s_chuck.chars().nth(i as usize) {
                    if c != '#' {
                        new_s.push(c);
                    }
                }
            } else if i != num_rows - 1 {
                // add the characters at the beginning and the end
                // to the vector
                // Filter out the added '#'
                if let Some(c) = s_chuck.chars().nth(i as usize) {
                    if c != '#' {
                        new_s.push(c);
                    }
                }

                if let Some(c) = s_chuck.chars().nth(s_chuck.len() - i as usize) {
                    if c != '#' {
                        new_s.push(c);
                    }
                }
            }
        }
    }
    new_s
}
