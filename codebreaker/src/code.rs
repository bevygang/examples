pub fn code_generate(count: usize, digits: u32) -> Option<Vec<char>> {
    let mut result = Vec::new();

    for _ in 0..count {
        let r = rand::random_range(1..=digits);
        let c = char::from_digit(r, digits +1)?;
        result.push(c);
    }

    Some(result)
}

pub fn code_input(input: &str, count: usize, digits: u32) -> Option<Vec<char>> {
    let chars = input.chars().collect::<Vec<_>>();

    if is_valid(&chars, count, digits) {
        Some(chars)
    } else {
        None
    }
}

pub fn code_eval(code: &[char], input: &[char]) -> Option<Vec<char>> {
    let n = code.len();

    if input.len() != n {
        return None;
    }

    let mut black_count: usize = 0;
    let mut white_count: usize = 0;
    let mut input_used = vec![false; n];
    let mut code_used = vec![false; n];

    for i in 0..n {
        if code[i] == input[i] {
            black_count += 1;
            input_used[i] = true;
            code_used[i] = true;
        }
    }

    for i in 0..n {
        if input_used[i] { continue; }
        for j in 0..n {
            if i != j && !code_used[j] && input[i] == code[j] {
                white_count += 1;
                // input_used[i] = true; // Same input will not be tested again
                code_used[j] = true;
                break;
            }
        }
    }

    let mut result = Vec::new();

    result.extend_from_slice(&vec!['S'; black_count]);
    result.extend_from_slice(&vec!['W'; white_count]);

    Some(result)
}

pub fn code_string(code: &[char]) -> String {
    code.iter().collect()
}

fn is_valid(code: &[char], count: usize, digits: u32) -> bool {
    for c in code.iter() {
        if !c.is_digit(digits + 1) || *c == '0' {
            return false;
        }
    }

    code.len() == count
}
