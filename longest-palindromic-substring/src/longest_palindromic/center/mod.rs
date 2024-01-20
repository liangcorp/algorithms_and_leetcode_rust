pub fn find_string(s: String) -> String {
    let mut new_s: Vec<char> = Vec::new();
    for c in s.chars() {
        new_s.push('#');
        new_s.push(c);
    }
    new_s.push('#');

    let mut palindrom_radii = vec![0; new_s.len()];
    let mut center = 0;
    let mut radius = 0;

    // if s.len() == 1 {
    //     return s;
    // }

    while center < new_s.len() {
        radius = 0;

        while center + radius < new_s.len() && new_s[center - radius] == new_s[center + radius] {
            radius += 1;

            if radius > center {
                break;
            }
        }

        palindrom_radii[center] = radius;
        center += 1;
    }

    center = *palindrom_radii.iter().max().unwrap();
    println!("{:?}, center {}", palindrom_radii, center);
    // println!(
    //     " result {}, char {}",
    //     center,
    //     &s.as_str()[center - radius - 1..center + radius]
    // );
    s.as_str()[center / 2 - radius / 2..center / 2 + radius / 2].to_string()
}
