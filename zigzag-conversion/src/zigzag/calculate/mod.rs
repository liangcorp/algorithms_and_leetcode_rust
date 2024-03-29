/// image the string are broken into different chucks of
/// size num_rows + num_rows - 2
///
/// create a loop that loops `num_rows` time
///
/// each loop go through the string and caluate the indexes
/// of character that fits the following logic:
///  - row 0 are push into the new string
///      - if string index % distance == index
///  - rest of the strings's head and tail are pushed into the new string
///     - (string index + num_rows index) % distance == 0
pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let mut new_s = String::new();
    let distance = num_rows + num_rows - 2;

    for i in 0..num_rows {
        for j in i as usize..s.len() {
            if (j as i32) % distance == i || (j as i32 + i) % distance == 0 {
                if let Some(c) = s.chars().nth(j) {
                    new_s.push(c);
                }
            }
        }
    }
    new_s
}
