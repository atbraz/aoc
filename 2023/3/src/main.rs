use std::env;
use std::fs::File;
use std::{
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Clone)]
struct Number {
    value: i32,
    row: usize,
    start_col: usize,
    end_col: usize,
}

#[allow(dead_code)]
enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    OnBlack,
    OnRed,
    OnGreen,
    OnYellow,
    OnBlue,
    OnMagenta,
    OnCyan,
    OnWhite,
    OnBrightBlack,
    OnBrightRed,
    OnBrightGreen,
    OnBrightYellow,
    OnBrightBlue,
    OnBrightMagenta,
    OnBrightCyan,
    OnBrightWhite,
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    None,
}

#[allow(dead_code)]
impl Color {
    fn wrap_code(&self) -> Option<&'static str> {
        let code = match self {
            // Regular colors (foreground)
            Color::Black => "30",
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
            // Bright colors (foreground)
            Color::BrightBlack => "90",
            Color::BrightRed => "91",
            Color::BrightGreen => "92",
            Color::BrightYellow => "93",
            Color::BrightBlue => "94",
            Color::BrightMagenta => "95",
            Color::BrightCyan => "96",
            Color::BrightWhite => "97",
            // Background colors
            Color::OnBlack => "40",
            Color::OnRed => "41",
            Color::OnGreen => "42",
            Color::OnYellow => "43",
            Color::OnBlue => "44",
            Color::OnMagenta => "45",
            Color::OnCyan => "46",
            Color::OnWhite => "47",
            Color::OnBrightBlack => "100",
            Color::OnBrightRed => "101",
            Color::OnBrightGreen => "102",
            Color::OnBrightYellow => "103",
            Color::OnBrightBlue => "104",
            Color::OnBrightMagenta => "105",
            Color::OnBrightCyan => "106",
            Color::OnBrightWhite => "107",
            // Styles
            Color::Bold => "1",
            Color::Dim => "2",
            Color::Italic => "3",
            Color::Underline => "4",
            Color::Blink => "5",
            Color::Reverse => "7",
            Color::None => return None,
        };
        Some(code)
    }

    fn wrap(&self, text: &str) -> String {
        if let Some(code) = self.wrap_code() {
            format!("\x1b[{code}m{text}\x1b[0m")
        } else {
            text.to_string()
        }
    }

    fn combine(colors: &[Color], text: &str) -> String {
        let codes: Vec<&str> = colors.iter().filter_map(Color::wrap_code).collect();

        if codes.is_empty() {
            return text.to_string();
        }

        format!("\x1b[{}m{text}\x1b[0m", codes.join(";"))
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn has_adjacent_symbol(grid: &[Vec<char>], number: &Number) -> Option<(usize, usize, char)> {
    let rows = grid.len();
    let cols = grid[0].len();

    for row in number.row.saturating_sub(1)..=(number.row + 1).min(rows - 1) {
        for col in number.start_col.saturating_sub(1)..=(number.end_col + 1).min(cols - 1) {
            if is_symbol(grid[row][col]) {
                return Some((row, col, grid[row][col]));
            }
        }
    }
    None
}

fn print_grid_section(
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

    println!(
        "\nChecking number {} @ [{}:{}] on row {}",
        number.value, number.start_col, number.end_col, number.row
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
        println!("{line}");
    }
}

fn print_colored_grid(grid: &[Vec<char>], part_numbers: &[Number], non_part_numbers: &[Number]) {
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

fn read_file(filename: &str) -> io::Result<BufReader<File>> {
    let path = Path::new(&filename);
    let file = File::open(path)?;
    Ok(io::BufReader::new(file))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).map_or("input", |s| s);

    match read_file(filename) {
        Ok(reader) => {
            let grid: Vec<Vec<char>> = reader
                .lines()
                .map_while(Result::ok)
                .map(|line| line.chars().collect())
                .collect();

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
                        "✓ Found symbol {} at position [{}, {}]",
                        symbol_pos.2, symbol_pos.0, symbol_pos.1
                    );
                    sum += number.value;
                    part_numbers.push(number.clone());
                } else {
                    print_grid_section(&grid, number, None);
                    println!("✗ No adjacent symbol found");
                    non_part_numbers.push(number.clone());
                }
                println!("Current sum: {sum}\n");
            }

            print_colored_grid(&grid, &part_numbers, &non_part_numbers);

            println!("=== Final Results ===");
            println!(
                "Part numbers: {:?}",
                part_numbers.iter().map(|n| n.value).collect::<Vec<_>>()
            );
            println!(
                "Non-part numbers: {:?}",
                non_part_numbers.iter().map(|n| n.value).collect::<Vec<_>>()
            );
            println!("Final sum: {sum}");
        }
        Err(e) => {
            eprintln!("Error reading file: {e}");
        }
    }
}
