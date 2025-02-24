use aoc_common_rust::color::Color;

#[derive(Debug, Clone)]
pub struct Number {
    pub value: i32,
    pub row: usize,
    pub start_col: usize,
    pub end_col: usize,
}

pub fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

pub fn has_adjacent_symbol(grid: &[Vec<char>], number: &Number) -> Option<(usize, usize, char)> {
    let rows = grid.len();
    let cols = grid[0].len();

    (number.row.saturating_sub(1)..=(number.row + 1).min(rows - 1)).find_map(|row| {
        (number.start_col.saturating_sub(1)..=(number.end_col + 1).min(cols - 1)).find_map(|col| {
            if is_symbol(grid[row][col]) {
                Some((row, col, grid[row][col]))
            } else {
                None
            }
        })
    })
}

pub fn print_grid_section(
    grid: &[Vec<char>],
    number: &Number,
    symbol_pos: Option<(usize, usize, char)>,
) {
    let rows = grid.len();
    let cols = grid[0].len();

    let start_row = number.row.saturating_sub(1);
    let end_row = (number.row + 1).min(rows - 1);
    let start_col = number.start_col.saturating_sub(1);
    let end_col = (number.end_col + 1).min(cols - 1);

    println!("----------------------------------------------------------");
    println!(
        "Number: {}\t| Coords: [{}:{}]\t| Row: {}",
        Color::Blue.wrap(&number.value.to_string()),
        Color::Blue.wrap(&number.start_col.to_string()),
        Color::Blue.wrap(&number.end_col.to_string()),
        Color::Blue.wrap(&number.row.to_string())
    );

    for (row, _) in grid.iter().enumerate().take(end_row + 1).skip(start_row) {
        let mut line = String::new();
        for col in start_col..=end_col {
            let c = grid[row][col];

            let color = if let Some((sym_row, sym_col, _)) = symbol_pos {
                if row == sym_row && col == sym_col {
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
) {
    for (row, line) in grid.iter().enumerate() {
        let mut colored_line = String::new();
        let mut current_col = 0;

        while current_col < line.len() {
            let c = line[current_col];

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
