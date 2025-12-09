# Advent of Code 2022

Solutions for [Advent of Code 2022](https://adventofcode.com/2022) written in **OCaml**.

## Stack

- **Language**: OCaml (≥ 5.3.0)
- **Build Tool**: Dune (≥ 3.17.0)
- **Package Manager**: opam
- **Task Runner**: Just (justfile for convenience)

## Project Structure

```
2022/
├── 1/
│   ├── main.ml       # Solution implementation
│   ├── dune          # Build configuration
│   ├── input         # Puzzle input
│   └── sample        # Sample input for testing
├── aoc-2022.opam     # Package definition
├── dune-project      # Dune project configuration
├── .ocamlformat      # Code formatting rules
└── justfile          # Build and run commands
```

## Commands

```bash
# Show all available commands
just

# Create a new day solution (generates template)
just new <day>

# Run with actual input
just run <day>

# Test with sample input
just test <day> [sample_file]

# Build a specific day
just build <day>

# Build all days
just build-all

# Format code
just fmt [day]

# Clean build artifacts
just clean
```

## Development Workflow

1. **Create new day**: `just new 7` creates directory with template files
2. **Add inputs**: Copy puzzle input to `7/input` and sample to `7/sample`
3. **Implement**: Edit `7/main.ml` with your solution
4. **Test**: `just test 7` to verify with sample input
5. **Format**: `just fmt 7` to auto-format your code
6. **Run**: `just run 7` to solve with actual input

## Code Structure

Each day's solution expects a single command line argument:

```ocaml
let () =
  let filename = Sys.argv.(1) in
  let lines = parse_input filename in
  Printf.printf "Part 1: %d\n" (part1 lines);
  Printf.printf "Part 2: %d\n" (part2 lines)
```

The template provides helper functions to get started:
- `parse_input`: Reads input file into a list of lines
- `part1`: Implement part 1 solution
- `part2`: Implement part 2 solution

## Intricacies

- **Common Library**: Uses `aoc-common-ocaml` for shared utilities
- **Public Names**: Each day has a public executable name: `aoc-2022-<day>`
- **Dune Package**: All days belong to the `aoc-2022` package
- **OCamlformat**: Code formatting rules are defined in `.ocamlformat`
- **Modern OCaml**: Uses OCaml 5.3+ features (effects, multicore support)
- **Single Package**: Unlike Rust/Cargo, all days are in one opam package
- **File Arguments**: Input files are passed as command line args, not via stdin
- **Dune Exec**: The justfile uses `dune exec` to build and run in one step
