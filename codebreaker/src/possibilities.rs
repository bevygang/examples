use crate::code::code_eval;

pub fn remaining(inputs: &[(Vec<char>, Vec<char>)], count: usize, digits: u32) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let mut current = vec!['1'; count];

    loop {
        if validate(&current, inputs) {
            result.push(current.clone());
        }

        if !next(&mut current, digits) {
            break;
        }
    }

    result
}

fn validate(candidate: &[char], inputs: &[(Vec<char>,Vec<char>)]) -> bool {
    for (input, eval) in inputs.iter() {
        let Some(candidate_eval) = code_eval(candidate, input) else { return false };

        if candidate_eval != *eval { return false; }
    }

    true
}

fn next(current: &mut Vec<char>, digits: u32) -> bool {
    for i in 0..current.len() {
        let c = current[i];
        let Some(mut digit) = c.to_digit(digits+1) else { return false; };

        digit += 1;
        if digit <= digits {
            current[i] = char::from_digit(digit, digits+1).unwrap();
            return true;
        }

        current[i] = '1';
    }

    false
}