use aoc_common_rust::color::Color;

#[derive(Debug, Clone)]
pub struct Number {
    pub value: i32,
    pub row: usize,
    pub start_col: usize,
    pub end_col: usize,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub value: char,
    pub row: usize,
    pub col: usize,
}

#[must_use]
pub fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

#[must_use]
pub fn has_adjacent_symbol(grid: &[Vec<char>], number: &Number) -> Option<Symbol> {
    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let row_start = number.row.saturating_sub(1);
    let row_end = (number.row + 1).min(n_rows - 1);
    let rows = row_start..=row_end;

    for row in rows {
        let col_start = number.start_col.saturating_sub(1);
        let col_end = (number.end_col + 1).min(n_cols - 1);
        let cols = col_start..=col_end;

        for col in cols {
            let char = grid[row][col];

            if is_symbol(char) {
                return Some(Symbol {
                    value: char,
                    row,
                    col,
                });
            };
        }
    }
    None
}

#[must_use]
pub fn has_adjacent_number(grid: &[Vec<char>], symbol: &Symbol) -> Option<Number> {
    let mut current_number = String::new();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let row_start = symbol.row.saturating_sub(1);
    let row_end = (symbol.row + 1).min(n_rows - 1);
    let rows = row_start..=row_end;

    for row in rows {
        current_number.clear();

        let col_start = symbol.col.saturating_sub(1);
        let col_end = (symbol.col + 1).min(n_cols - 1);
        let cols = col_start..=col_end;

        for col in cols {
            let c = grid[row][col];
            if c.is_ascii_digit() {
                current_number.push(c);
                println!("Found digit {c}");
                println!("Curr number: {current_number}");
            } else if !current_number.is_empty() {
                println!("Got to the end of a number, returning {current_number}");
                return Some(Number {
                    value: current_number.parse().unwrap(),
                    row,
                    start_col: col - current_number.len(),
                    end_col: col - 1,
                });
            }
        }
        if !current_number.is_empty() {
            println!("Got to the end of the line, returning {current_number}");
            return Some(Number {
                value: current_number.parse().unwrap(),
                row,
                start_col: n_cols - current_number.len(),
                end_col: n_cols - 1,
            });
        }
    }
    println!("Did not find any numbers");
    None
}

#[must_use]
pub fn read_numbers(grid: &[Vec<char>]) -> Vec<Number> {
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
    numbers
}

pub fn print_grid_section(grid: &[Vec<char>], number: &Number, symbol: Option<&Symbol>) {
    let rows = grid.len();
    let cols = grid[0].len();

    let start_row = number.row.saturating_sub(1);
    let end_row = (number.row + 1).min(rows - 1);
    let start_col = number.start_col.saturating_sub(1);
    let end_col = (number.end_col + 1).min(cols - 1);

    println!(
        "Number: {} @ [{}, {}:{}]\n",
        Color::Blue.wrap(&number.value.to_string()),
        Color::Blue.wrap(&number.row.to_string()),
        Color::Blue.wrap(&number.start_col.to_string()),
        Color::Blue.wrap(&number.end_col.to_string()),
    );

    for (row, _) in grid.iter().enumerate().take(end_row + 1).skip(start_row) {
        let mut line = String::new();
        for col in start_col..=end_col {
            let c = grid[row][col];

            let color = if let Some(Symbol {
                value: _,
                row: symbol_row,
                col: symbol_col,
            }) = symbol
            {
                if row == *symbol_row && col == *symbol_col {
                    Color::Red
                } else if row == number.row && (col >= number.start_col && col <= number.end_col) {
                    Color::Green
                } else {
                    Color::None
                }
            } else if row == number.row && (col >= number.start_col && col <= number.end_col) {
                Color::Green
            } else {
                Color::None
            };

            line.push_str(&color.wrap(&c.to_string()));
        }
        println!("\t{line}");
    }
}

pub fn print_colored_grid(
    grid: &[Vec<char>],
    part_numbers: &[Number],
    non_part_numbers: &[Number],
    mult_part_numbers: Option<&[Number]>,
) {
    for (row, line) in grid.iter().enumerate() {
        let mut colored_line = row.to_string() + ":\t";
        let mut current_col = 0;

        while current_col < line.len() {
            let c = line[current_col];

            let mult_number_format = if let Some(mult_numbers) = mult_part_numbers {
                mult_numbers
                    .iter()
                    .find(|n| {
                        n.row == row && current_col >= n.start_col && current_col <= n.end_col
                    })
                    .map(|number| (number, &[Color::OnWhite, Color::Cyan]))
            } else {
                None
            };

            if let Some((number, colors)) = mult_number_format {
                let num_str: String = (number.start_col..=number.end_col)
                    .map(|col| line[col])
                    .collect();
                colored_line.push_str(&Color::combine(colors, &num_str));
                current_col = number.end_col + 1;
                continue;
            }

            let number_types = [(part_numbers, Color::Green), (non_part_numbers, Color::Red)];
            let number_format = number_types.iter().find_map(|(numbers, color)| {
                numbers
                    .iter()
                    .find(|n| {
                        n.row == row && current_col >= n.start_col && current_col <= n.end_col
                    })
                    .map(|number| (number, color))
            });

            if let Some((number, color)) = number_format {
                let num_str: String = (number.start_col..=number.end_col)
                    .map(|col| line[col])
                    .collect();

                colored_line.push_str(&color.wrap(&num_str));
                current_col = number.end_col + 1;
                continue;
            }

            if is_symbol(c) {
                colored_line.push_str(&Color::Yellow.wrap(&c.to_string()));
            } else {
                colored_line.push(c);
            }

            current_col += 1;
        }
        println!("{colored_line}");
    }
    println!();
}
