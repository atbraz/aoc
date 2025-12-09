# Advent of Code 2024

Solutions for [Advent of Code 2024](https://adventofcode.com/2024) written in **C++**.

## Stack

- **Language**: C++17
- **Compiler**: g++ (via Homebrew)
- **Build Tool**: Just (justfile for task running)

## Project Structure

```
2024/
├── 1/
│   ├── main.cpp    # Solution implementation
│   ├── input       # Puzzle input
│   └── sample_1    # Sample input for testing
├── 2/
│   └── ...
└── justfile        # Build and run commands
```

## Commands

```bash
# Show all available commands
just

# Create a new day solution (generates template)
just new <day>

# Build a specific day
just build <day>

# Run with actual input
just run <day>

# Test with sample input
just test <day>

# Clean compiled binaries
just clean [day]
```

## Development Workflow

1. **Create new day**: `just new 5` creates the directory structure with template
2. **Add inputs**: Copy puzzle input to `5/input` and sample to `5/sample_1`
3. **Implement**: Edit `5/main.cpp` with your solution
4. **Test**: `just test 5` to verify with sample input
5. **Run**: `just run 5` to solve with actual input

## Code Structure

Each day's solution follows this pattern:

```cpp
std::vector<std::string> parse_input(std::string input_file);
int solve_part_1(const std::vector<std::string>& lines);
int solve_part_2(const std::vector<std::string>& lines);
```

The input filename is passed via stdin (handled by the justfile).

## Intricacies

- **Input Method**: Input file path is read from stdin, not command line args
  - This is why the justfile uses `echo "day/input" | ./day/day_N`
- **Compilation**: Each day compiles to its own binary (`day_N`)
- **No Central Workspace**: Each day is independent, no shared Cargo.toml equivalent
- **Compiler Flags**: `-std=c++17 -Wall -Wextra -O2` for safety and optimization
