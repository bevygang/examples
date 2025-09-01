use crate::code::code_string;
use crate::possibilities::remaining;
use crate::solver::next_input;

pub enum ParseAction {
    Break, Continue, Input(String), None
}

pub fn parse_input(input: &str, code: &[char], inputs: &[(Vec<char>,Vec<char>)], count: usize, digits: u32) -> ParseAction {
    match input.to_lowercase().as_str() {
        "x" | "ende" | "exit" => {
            println!("Der Code war {}", code_string(code));
            ParseAction::Break
        },
        "c" | "anzahl" | "count" => {
            print_remaining_count(inputs, count, digits);
            ParseAction::Continue
        },
        "r" | "verbleibend" | "remaining" => {
            print_remaining(inputs, count, digits);
            ParseAction::Continue
        },
        "n" | "nächster" | "next" => {
            match next_input(inputs) {
                Some(next_input) => ParseAction::Input(next_input.iter().collect()),
                None => {
                    print_remaining_count(inputs, count, digits);
                    ParseAction::Continue
                }
            }
        },
        _ => ParseAction::None
    }
}

fn print_remaining_count(inputs: &[(Vec<char>, Vec<char>)], count: usize, digits: u32) {
    let remaining = remaining(inputs, count, digits);
    println!("n={}", remaining.len());
}

fn print_remaining(inputs: &[(Vec<char>, Vec<char>)], count: usize, digits: u32) {
    let remaining = remaining(&inputs, count, digits);
    let mut count = 0;

    for pattern in remaining.iter() {
        print!("{} ", code_string(pattern));
        count += 1;
        if count > 9 {
            println!();
            count = 0;
        }
    }

    if count != 0 {
        println!();
    }
}
