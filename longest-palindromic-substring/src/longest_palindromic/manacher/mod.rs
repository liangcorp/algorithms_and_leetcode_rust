pub fn find_string(s: String) -> String {
    let mut new_s: Vec<char>;

    //  expend the string with '#'
    new_s = Vec::from_iter(s.chars())
        .into_iter()
        .flat_map(|x| ['#', x])
        .collect();
    new_s.push('#');
    new_s.push('$');
    new_s.insert(0, '@');

    let mut p_radii = vec![0; new_s.len()];

    let mut c = 1; // center of palnidormic
    let mut r = 0; // right limit of palnidorm
    let mut i_mirror; // left limit of palanidrom

    let mut max_radius = 0;
    let mut max_index = 0;

    for i in 1..(new_s.len() - 1) {
        // find the corresponding letter in the palidrome subString
        i_mirror = c - (i - c);

        // if current char is inside the border of
        //  the largest palidrome
        if r > i {
            p_radii[i] = if r - i < p_radii[i_mirror] {
                // if mirror character's palidrome
                // expends beyond the border of the
                // largest palidrome
                r - i // palidrome of current character is at least
                      // the size of (border - index)
            } else {
                p_radii[i_mirror] // mirror property
            }
        }

        // expanding around index
        // starting point depending on (border - index) if applicable
        while new_s[i + 1 + p_radii[i]] == new_s[i - 1 - p_radii[i]] {
            p_radii[i] += 1;
        }

        // Update center and border in case if the palindrome
        // centered at i expands past border
        // (i.e. a larger palidrome),
        if (i + p_radii[i]) > r {
            c = i; // next c = i
            r = i + p_radii[i];
        }

        // Keep track of the index of char with the
        // largest palidrome and it's radius
        if max_radius < p_radii[i] {
            max_radius = p_radii[i];
            max_index = i;
        }
    }

    // Find the longest palindromic substring using index of character
    //      with the max palindromic radius
    // Filter out the '#'
    let result_sv: Vec<&char> = new_s.as_slice()[max_index - max_radius..max_index + max_radius]
        .iter()
        .filter(|x| **x != '#')
        .collect();

    String::from_iter(result_sv)
}
