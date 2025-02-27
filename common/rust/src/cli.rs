use crate::color::Color;
use crate::input::InputError;
use std::env;
use std::process;

/**
Runs the specified solver with the given command-line arguments

# Arguments
* `part_1` - A function that solves part 1
* `part_2` - A function that solves part 2

# Example

```
use aoc_common_rust::cli;

fn part1_solver(filename: &str) -> Result<i32, aoc_common_rust::input::InputError> {
    // solution implementation
    Ok(42)
}

fn part2_solver(filename: &str) -> Result<i32, aoc_common_rust::input::InputError> {
    // solution implementation
    Ok(24)
}

fn main() {
    cli::run_funcs(part1_solver, part2_solver);
}
```
*/
pub fn run<F1, F2>(part_1: F1, part_2: F2)
where
    F1: Fn(&str) -> Result<i32, InputError>,
    F2: Fn(&str) -> Result<i32, InputError>,
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <part> [input_file]", args[0]);
        eprintln!("  part: 1 or 2");
        eprintln!("  input_file: optional, defaults to 'input'");
        process::exit(1);
    }

    let part = &args[1];
    let filename = args.get(2).map_or("input", |s| s);

    let result = match part.as_str() {
        "1" => part_1(filename),
        "2" => part_2(filename),
        _ => {
            eprintln!("Invalid part number. Use 1 or 2");
            process::exit(1);
        }
    };

    match result {
        Ok(sum) => println!("Solution: {}", Color::Blue.wrap(&sum.to_string())),
        Err(e) => {
            eprintln!("Error: {e:#?}");
            process::exit(1);
        }
    }
}
