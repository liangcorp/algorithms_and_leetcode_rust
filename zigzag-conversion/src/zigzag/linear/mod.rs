pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let num_rows = num_rows as usize;
    let mut res = String::new();
    let s_chars: Vec<char> = s.chars().collect();

    for r in 0..num_rows {
        let increment = 2 * (num_rows - 1);
        let mut i = r;

        while i < s_chars.len() {
            res.push(s_chars[i]);

            if r > 0 && r < num_rows - 1 {
                let next_index = i + increment - 2 * r;

                if next_index < s_chars.len() {
                    res.push(s_chars[next_index]);
                }
            }
            i += increment;
        }
    }
    res
}
