pub fn find_string(s: String) -> String {
    let mut new_s: Vec<char>;

    if s.len() % 2 == 0 {
        // Make the string with odd length
        // Insert '#' and beginning, end and inbetween every charater in the string
        new_s = Vec::from_iter(s.chars())
            .into_iter()
            .flat_map(|x| ['#', x])
            .collect();
        new_s.push('#');
    } else {
        new_s = Vec::from_iter(s.chars());
    }

    let mut left = 0;
    let mut right = 0;
    let mut center = 0;
    let mut radius;

    while right < new_s.len() {
        radius = 0;

        // Keep expending the radius from center character as long as two char matches
        while center + radius < new_s.len() && new_s[center - radius] == new_s[center + radius] {
            radius += 1;

            // Hit the end of string
            if radius > center {
                break;
            }
        }

        if center - radius - 1 < 0 {
            left = 0
        } else {
            left = center - radius - 1;
        }
        right = center + radius + 1;

        // Use the next char as center
        center = right;
    }

    // Find the longest palindromic substring using index of character
    //      with the max palindromic radius
    // Filter out the '#'
    let result_sv: Vec<&char> = if s.len() % 2 == 0 {
        new_s.as_slice()[left..right]
            .iter()
            .filter(|x| **x != '#')
            .collect()
    } else {
        new_s.as_slice()[left..right].iter().collect()
    };

    println!("{:?}", result_sv);
    String::from_iter(result_sv)
}
