use std::io::{stdout, BufWriter};
use ferris_says::say;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("Usage: greeter message");
        std::process::exit(1);
    }

    let mut writer = BufWriter::new(stdout());
    let _ = say(&args[1], 80, &mut writer);
}
