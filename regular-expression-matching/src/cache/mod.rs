use std::collections::HashMap;

pub fn is_match(s: String, p: String) -> bool {
    let mut map: HashMap<(usize, usize), bool> = HashMap::new();
    match_string(&s, &p, 0, 0, &mut map)
}

pub fn match_string(
    s: &String,
    p: &String,
    i: usize,
    j: usize,
    map: &mut HashMap<(usize, usize), bool>,
) -> bool {
    if map.contains_key(&(i, j)) {
        if let Some(&result) = map.get(&(i, j)) {
            return result;
        }
    }

    if i >= s.len() && j >= p.len() {
        return true;
    }

    if j >= p.len() {
        return false;
    }

    let is_char_match = i < s.len()
        && (s.chars().nth(i).unwrap() == p.chars().nth(j).unwrap()
            || p.chars().nth(j).unwrap() == '.');

    if j + 1 < p.len() && p.chars().nth(j + 1).unwrap() == '*' {
        let calculated_val = match_string(s, p, i, j + 2, map)
            || (is_char_match && match_string(s, p, i + 1, j, map));

        map.insert((i, j), calculated_val);

        if let Some(&result) = map.get(&(i, j)) {
            return result;
        }
    }

    if is_char_match {
        let calculated_val = match_string(s, p, i + 1, j + 1, map);

        map.insert((i, j), calculated_val);

        if let Some(&result) = map.get(&(i, j)) {
            return result;
        }
    }

    false
}
