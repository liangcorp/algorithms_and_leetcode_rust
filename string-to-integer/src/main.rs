pub fn my_atoi(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut clean_s: Vec<char> = s
        .trim()
        .split(' ')
        .next()
        .unwrap_or("")
        .chars()
        .filter(|&x| x != ' ')
        .collect();
    if clean_s.is_empty() {
        return 0;
    }
    let mut is_negative = false;
    let mut temp_s: Vec<char> = vec![];

    if clean_s[0] == '-' {
        is_negative = true;
        clean_s = clean_s[1..].to_vec();
    } else if clean_s[0] == '+' {
        is_negative = false;
        clean_s = clean_s[1..].to_vec();
    }

    for c in clean_s.iter() {
        if c.is_numeric() {
            temp_s.push(*c);
        } else {
            break;
        }
    }

    if temp_s.is_empty() {
        return 0;
    }

    let result_s: String = if is_negative {
        format!("{}{}", '-', String::from_iter(temp_s))
    } else {
        String::from_iter(temp_s)
    };

    match result_s.parse::<i32>() {
        Ok(result) => result,
        Err(_) => {
            if is_negative {
                -2147483648
            } else {
                2147483647
            }
        }
    }
}

fn main() {
    println!("{}", my_atoi(String::from("-91283472332")));
}
