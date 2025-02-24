use aoc_2023_3::{part_1, part_2};
use aoc_common_rust::{color::Color, input::InputError};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -p aoc-2023-3 <part> [input_file]");
        eprintln!("  part: 1 or 2");
        eprintln!("  input_file: optional, defaults to 'input'");
        std::process::exit(1);
    }

    let part = &args[1];
    let filename = args.get(2).map_or("input", |s| s);

    let result: Result<i32, InputError> = match part.as_str() {
        "1" => part_1::solve(filename),
        "2" => part_2::solve(filename),
        _ => {
            eprintln!("Invalid part number. Use 1 or 2");
            std::process::exit(1);
        }
    };

    match result {
        Ok(sum) => println!("Solution: {}", Color::Blue.wrap(&sum.to_string())),
        Err(e) => {
            eprintln!("Error: {e:#?}");
            std::process::exit(1);
        }
    }
}
