use crate::utils::{has_adjacent_symbol, print_colored_grid, print_grid_section, Number};
use aoc_common_rust::{
    color::Color,
    input::{InputError, InputReader},
};

/**
.

# Panics

Panics if .

# Errors

This function will return an error if .
*/
pub fn solve(filename: &str) -> Result<i32, InputError> {
    let grid = InputReader::as_char_grid(filename)?;
    println!("Grid size: {} x {}", grid.len(), grid[0].len());

    let mut numbers = Vec::new();
    let mut current_number = String::new();

    for (row, line) in grid.iter().enumerate() {
        current_number.clear();
        for (col, &c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                current_number.push(c);
            } else if !current_number.is_empty() {
                numbers.push(Number {
                    value: current_number.parse().unwrap(),
                    row,
                    start_col: col - current_number.len(),
                    end_col: col - 1,
                });
                current_number.clear();
            }
        }
        if !current_number.is_empty() {
            numbers.push(Number {
                value: current_number.parse().unwrap(),
                row,
                start_col: line.len() - current_number.len(),
                end_col: line.len() - 1,
            });
            current_number.clear();
        }
    }

    println!("Found {} numbers in total", numbers.len());

    let mut sum = 0;
    let mut part_numbers = Vec::new();
    let mut non_part_numbers = Vec::new();

    for number in &numbers {
        if let Some(symbol_pos) = has_adjacent_symbol(&grid, number) {
            print_grid_section(&grid, number, Some(symbol_pos));
            println!(
                "{} Found symbol {} at position [{}, {}]",
                Color::Green.wrap("✓"),
                Color::Red.wrap(&symbol_pos.2.to_string()),
                Color::Blue.wrap(&symbol_pos.0.to_string()),
                Color::Blue.wrap(&symbol_pos.1.to_string())
            );
            sum += number.value;
            part_numbers.push(number.clone());
        } else {
            print_grid_section(&grid, number, None);
            println!("{} No adjacent symbol found", Color::Red.wrap("✗"));
            non_part_numbers.push(number.clone());
        }
        println!("Current sum: {}", Color::Yellow.wrap(&sum.to_string()));
        println!("----------------------------------------------------------\n");
    }

    print_colored_grid(&grid, &part_numbers, &non_part_numbers);

    Ok(sum)
}
