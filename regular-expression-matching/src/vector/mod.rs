/// Very slow for some reason
pub fn is_match(s: String, p: String) -> bool {
    let s_vec: Vec<char> = s.chars().collect();
    let p_vec: Vec<char> = p.chars().collect();

    is_match_vector(s_vec, p_vec)
}

pub fn is_match_vector(s_vec: Vec<char>, p_vec: Vec<char>) -> bool {
    if p_vec.is_empty() {
        return s_vec.is_empty();
    }

    println!("{:?} {:?}", s_vec, p_vec);

    let is_first_match = !s_vec.is_empty() && (s_vec[0] == p_vec[0] || p_vec[0] == '.');

    if p_vec.len() >= 2 && p_vec[1] == '*' {
        is_match_vector(s_vec.clone(), p_vec[2..].to_vec())
            || (is_first_match && is_match_vector(s_vec[1..].to_vec(), p_vec))
    } else {
        is_first_match && is_match_vector(s_vec[1..].to_vec(), p_vec[1..].to_vec())
    }
}
