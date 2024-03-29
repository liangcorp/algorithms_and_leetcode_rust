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
    // Using rust String
    if num_rows == 1 {
        return s;
    }

    let mut new_s = String::new();
    let distance = (num_rows * 2 - 2) as usize;
    let mut head_index: usize; // head of string chuck without [0]
    let mut tail_index: usize; // tail of string chuck

    for row_no in 0..num_rows as usize {
        head_index = row_no;

        while head_index < s.len() {
            // Push head of the string chuck
            if let Some(c) = s.chars().nth(head_index) {
                new_s.push(c);
            }
            if row_no > 0 && row_no < num_rows as usize - 1 {
                tail_index = head_index + distance - row_no * 2;

                // push the tail of the string chuck
                if tail_index < s.len() {
                    if let Some(c) = s.chars().nth(tail_index) {
                        new_s.push(c);
                    }
                }
            }
            // Speeds up the solution by moving forward
            // by `distance`
            head_index += distance;
        }
    }
    new_s
}

pub fn convert_vector(s: String, num_rows: i32) -> String {
    // use Rust Vector
    if num_rows == 1 {
        return s;
    }

    let old_s: Vec<char> = s.chars().collect();

    let mut new_s = String::new();
    let distance = (num_rows * 2 - 2) as usize;
    let mut head_index: usize; // head of string chuck without [0]
    let mut tail_index: usize; // tail of string chuck

    for row_no in 0..num_rows as usize {
        head_index = row_no;

        while head_index < old_s.len() {
            // Push head of the string chuck
            new_s.push(old_s[head_index]);

            if row_no > 0 && row_no < num_rows as usize - 1 {
                tail_index = head_index + distance - row_no * 2;

                // push the tail of the string chuck
                if tail_index < old_s.len() {
                    new_s.push(old_s[tail_index]);
                }
            }
            // Speeds up the solution by moving forward
            // by `distance`
            head_index += distance;
        }
    }
    new_s
}
