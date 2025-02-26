use crate::utils::{
    has_adjacent_symbol, print_colored_grid, print_grid_section, read_numbers, Number,
};
use aoc_common_rust::{
    color::Color,
    input::{InputError, InputReader},
};

pub fn solve(filename: &str) -> Result<i32, InputError> {
    let grid = InputReader::as_char_grid(filename)?;
    println!("Grid size: {} x {}", grid.len(), grid[0].len());

    let numbers: Vec<Number> = read_numbers(&grid);
    println!("Found {} numbers in total", numbers.len());

    let mut sum = 0;
    let mut part_numbers = Vec::new();
    let mut non_part_numbers = Vec::new();

    for number in &numbers {
        if let Some(symbol) = has_adjacent_symbol(&grid, number) {
            sum += number.value;
            part_numbers.push(number.clone());

            print_grid_section(&grid, number, Some(&symbol));
            print!(
                "\n{} Found symbol {} at position [{}, {}]",
                Color::Green.wrap("✓"),
                Color::Red.wrap(&symbol.value.to_string()),
                Color::Blue.wrap(&symbol.row.to_string()),
                Color::Blue.wrap(&symbol.col.to_string())
            );
        } else {
            non_part_numbers.push(number.clone());

            print_grid_section(&grid, number, None);
            print!("\n{} No adjacent symbol found", Color::Red.wrap("✗"));
        }
        print!("\t| Current sum: {}", Color::Yellow.wrap(&sum.to_string()));
        println!("\n----------------------------------------------------------\n");
    }

    print_colored_grid(&grid, &part_numbers, &non_part_numbers, None);

    Ok(sum)
}
