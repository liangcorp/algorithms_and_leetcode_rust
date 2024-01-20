pub fn find_string(s: String) -> String {
    // Insert '#' and beginning, end and inbetween every charater in the string
    let mut new_s: Vec<char> = Vec::from_iter(s.chars())
        .into_iter()
        .flat_map(|x| ['#', x])
        .collect();
    new_s.push('#');

    let mut center = 0;
    let mut radius;
    let mut max_radius = 0;
    let mut max_index = 0;

    while center < new_s.len() {
        radius = 0;

        // Keep expending the radius from center character as long as two char matches
        while center + radius < new_s.len() && new_s[center - radius] == new_s[center + radius] {
            radius += 1;

            // Hit the end of string
            if radius > center {
                break;
            }
        }

        // Store the index of character with max palindromic radius
        if max_radius < radius {
            max_radius = radius;
            max_index = center;
        }

        // Use the next char as center
        center += 1;
    }

    // Find the longest palindromic substring using index of character
    //      with the max palindromic radius
    // Filter out the '#'
    let result_sv: Vec<&char> = new_s.as_slice()
        [max_index - (max_radius - 1)..max_index + (max_radius - 1)]
        .iter()
        .filter(|x| **x != '#')
        .collect();

    String::from_iter(result_sv)
}
