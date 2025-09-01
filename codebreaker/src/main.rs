mod code;
mod possibilities;
mod solver;
mod commands;

use code::{code_generate, code_input, code_eval, code_string};
use commands::{parse_input, ParseAction};

use std::io::Write;

fn main() {
    let count = 4;
    let digits = 6;
    let mut inputs = Vec::new();

    let Some(code) = code_generate(count, digits) else {
        println!("Code konnte nicht erstellt werden.");
        return;
    };

    loop
    {
        print!("{:>2}", inputs.len()+1);

        let Some(mut input) = get_input() else {
            println!("Fehler bei der Eingabe.");
            break;
        };

        match parse_input(&input, &code, &inputs, count, digits) {
            ParseAction::Break => break,
            ParseAction::Continue => continue,
            ParseAction::Input(next_input) => input = next_input,
            ParseAction::None => {}
        }

        let Some(input) = code_input(&input, count, digits) else {
            println!("Eingabe ungültig.");
            continue;
        };

        let Some(eval) = code_eval(&code, &input) else {
            println!("Fehler bei der Auswertung");
            break;
        };

        println!("{} -> {}", code_string(&input), code_string(&eval));

        if input == code {
            break;
        }

        inputs.push((input,eval));
    }
}

pub fn get_input() -> Option<String> {
    let mut buffer = String::new();

    print!("> ");
    std::io::stdout().flush().ok()?;
    std::io::stdin().read_line(&mut buffer).ok()?;

    Some(buffer.trim().to_string())
}

