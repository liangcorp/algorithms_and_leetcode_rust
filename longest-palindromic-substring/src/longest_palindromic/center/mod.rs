pub fn find_string(s: String) -> String {
    // Insert '#' and beginning, end and inbetween every charater in the string
    let mut new_s: Vec<char> = Vec::from_iter(s.chars())
        .into_iter()
        .flat_map(|x| ['#', x])
        .collect();
    new_s.push('#');

    let mut palindrom_radii = vec![0; new_s.len()];
    let mut center = 0;
    let mut radius;

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

        // Store the palindromic radius in the vector
        palindrom_radii[center] = radius;

        // Use the next char as center
        center += 1;
    }

    let mut max_radius = 0;
    let mut max_index = 0;

    for (i, m) in palindrom_radii.iter().enumerate() {
        if max_radius < *m {
            max_radius = *m;
            max_index = i;
        }
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
