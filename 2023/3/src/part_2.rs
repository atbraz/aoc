use crate::utils::{find_all_adjacent_numbers, Symbol};
use common::{
    color::Color,
    input::{InputError, InputReader},
};

pub fn solve(filename: &str) -> Result<i32, InputError> {
    let grid = InputReader::as_char_grid(filename)?;
    let mut sum = 0;

    let mut gear_symbols = Vec::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, &c) in line.iter().enumerate() {
            if c == '*' {
                gear_symbols.push(Symbol {
                    value: '*',
                    row,
                    col,
                });
            }
        }
    }

    for symbol in gear_symbols {
        let adjacent_numbers = find_all_adjacent_numbers(&grid, &symbol);

        if adjacent_numbers.len() == 2 {
            let gear_ratio = adjacent_numbers[0].value * adjacent_numbers[1].value;
            sum += gear_ratio;
            println!(
                "Found gear at [{}, {}] with ratio {}",
                Color::Blue.wrap(&symbol.row.to_string()),
                Color::Blue.wrap(&symbol.col.to_string()),
                Color::Blue.wrap(&gear_ratio.to_string())
            );
        }
    }

    // for number in &numbers {
    //     println!("───────────────────────────────────┐");
    //     if let Some(symbol) = has_adjacent_symbol(&grid, number) {
    //         print_grid_section(&grid, number, Some(&symbol));
    //         println!(
    //             "\n{} Found {}",
    //             Color::Green.wrap("✓"),
    //             Color::Red.wrap(&symbol.value.to_string()),
    //         );
    //
    //         let mut multiplier: i32 = 1;
    //         if symbol.value == '*' {
    //             if let Some(multiplier_number) = has_adjacent_number(&grid, &symbol) {
    //                 multiplier = multiplier_number.value;
    //                 mult_part_numbers.push(number.clone());
    //                 mult_part_numbers.push(multiplier_number.clone());
    //
    //                 println!(
    //                     "\n\t{} x {} @ [{}, {}:{}]",
    //                     Color::Green.wrap(&number.value.to_string()),
    //                     Color::Green.wrap(&multiplier_number.value.to_string()),
    //                     Color::Blue.wrap(&multiplier_number.row.to_string()),
    //                     Color::Blue.wrap(&multiplier_number.start_col.to_string()),
    //                     Color::Blue.wrap(&multiplier_number.end_col.to_string())
    //                 );
    //                 println!(
    //                     "\t= {}",
    //                     Color::Green.wrap(&(number.value * multiplier_number.value).to_string())
    //                 );
    //             } else {
    //                 println!("\tNo adjacent number found");
    //             }
    //         }
    //         sum += number.value * multiplier;
    //         part_numbers.push(number.clone());
    //     } else {
    //         non_part_numbers.push(number.clone());
    //
    //         print_grid_section(&grid, number, None);
    //         println!("\n{} No adjacent symbol found", Color::Red.wrap("✗"));
    //     }
    //     println!("\tCurrent sum: {}", Color::Yellow.wrap(&sum.to_string()));
    //     println!("───────────────────────────────────┘");
    // }

    // print_colored_grid(
    //     &grid,
    //     &part_numbers,
    //     &non_part_numbers,
    //     Some(&mult_part_numbers),
    // );

    Ok(sum)
}
