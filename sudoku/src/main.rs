use std::io::stdin;

fn main() {
    loop {
        let input = read_line();

        if input.chars().count() < 81 { break; }

        println!();
        print_sudoku(&input);
        println!();
    }
}

fn read_line() -> String {
    let reader = stdin();
    let mut input = String::new();
    let result : Result<usize, _> = reader.read_line(&mut input);

    if result.is_err() { return "".to_string(); }

    input
}

fn print_sudoku(input: &str) {
    let mut row = 0;
    let mut col = 0;

    for c in input.chars() {
        if row % 4 == 0 {
            print_horizontal();
            row += 1;
        }

        print_cell_prefix(col);
        print_cell(c);

        col += 1;

        if col >= 9 {
            print_end_of_line();
            col = 0;
            row += 1;

            if row >= 12 { break; }
        }
    }

    print_horizontal();
}

fn print_horizontal() {
    println!("+---+---+---+---+---+---+---+---+---+");
}

fn print_cell_prefix(col: u32) {
    if col == 0 {
        print!("| ");
    } else if col % 3 == 0 {
        print!(" | ");
    } else {
        print!(" + ");
    }
}

fn print_cell(c: char) {
    if c == '.' {
        print!(" ");
    } else {
        print!("{c}");
    }
}

fn print_end_of_line() {
    println!(" |");
}
