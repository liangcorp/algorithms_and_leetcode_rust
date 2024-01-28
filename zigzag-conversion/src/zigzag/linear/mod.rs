/// image the string are broken into different chucks of
/// size num_rows + num_rows - 2
///
/// create a loop that loops `num_rows` time
///
/// each loop go through the string and caluate the indexes
/// of character that fits the following logic:
///  - row 0 are push into the new string
///      - if string index % distance == index
///  - Find the index of head and tail of string that are row > 0
///     i.e. (string index + num_rows index) % distance == 0
///     Push the head and tail into the new string
pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let mut new_s = String::new();
    let distance = (num_rows * 2 - 2) as usize;
    let mut head_index: usize;

    for i in 0..num_rows as usize {
        head_index = i;
        while head_index < s.len() {
            if (head_index + i) % distance == 0 {
                if let Some(c) = s.chars().nth(head_index) {
                    new_s.push(c);
                }
            } else if head_index % distance == i {
                let tail_index = head_index + distance - 2 * i;
                if let Some(c) = s.chars().nth(head_index) {
                    new_s.push(c);
                }
                if let Some(c) = s.chars().nth(tail_index) {
                    new_s.push(c);
                }
            }
            head_index += distance;
        }
    }
    new_s
}
