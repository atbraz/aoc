# Advent of Code 2024 - C++ Solutions

This repository contains my C++ solutions for [Advent of Code 2024](https://adventofcode.com/2024).

## Project Structure

```
2024/
├── Makefile           # Build system for compiling solutions
├── new-day.sh         # Script to scaffold new day directories
├── run.sh             # Quick run script
├── cpp_utils/         # Shared utility functions
│   ├── parse_input.h
│   └── parse_input.cpp
├── 1/                 # Day 1 solution
│   ├── main.cpp
│   ├── input
│   ├── sample_1
│   └── day_1          # Compiled binary (generated)
├── 2/                 # Day 2 solution
│   └── ...
└── ...
```

## Requirements

- C++ compiler with C++17 support (g++ or clang++)
- Make
- Bash (for helper scripts)

## Quick Start

### Creating a New Day

To scaffold a new day's solution:

```bash
./new-day.sh <day_number>
```

Example:
```bash
./new-day.sh 5
```

This creates:
- `5/main.cpp` - Solution template with `parse_input`, `solve_part_1`, and `solve_part_2` functions
- `5/input` - Empty file for your puzzle input
- `5/sample_1` - Empty file for sample/test input

### Writing Your Solution

1. Add your puzzle input to `<day>/input` (copy from Advent of Code website)
2. Add the sample input to `<day>/sample_1` (from the problem description)
3. Open `<day>/main.cpp` and implement:
   - `solve_part_1()` - Part 1 solution
   - `solve_part_2()` - Part 2 solution
   - Modify `parse_input()` if needed for custom parsing

### Running Solutions

#### Quick Run (Recommended)

```bash
# Run with actual input
./run.sh <day>

# Run with sample input
./run.sh <day> sample
```

Examples:
```bash
./run.sh 1          # Run day 1 with input
./run.sh 1 sample   # Run day 1 with sample_1
```

#### Using Make

```bash
# Compile a specific day
make <day>

# Compile and run with input file
make run DAY=<day>

# Compile and run with sample file
make test DAY=<day>

# Compile all days
make all

# Clean compiled binaries
make clean          # Clean all
make clean DAY=<day> # Clean specific day
```

Examples:
```bash
make 1              # Compile day 1
make run DAY=1      # Compile and run day 1 with input
make test DAY=1     # Compile and run day 1 with sample
make clean DAY=1    # Remove day 1 binary
```

## Solution Template

Each day follows this structure:

```cpp
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

std::vector<std::string> parse_input(std::string input_file) {
    // Parse input file and return data structure
    // Customize this based on the day's requirements
}

int solve_part_1(const std::vector<std::string>& lines) {
    // Implement part 1 solution
    return 0;
}

int solve_part_2(const std::vector<std::string>& lines) {
    // Implement part 2 solution
    return 0;
}

int main() {
    std::string input_file;
    std::cin >> input_file;

    std::vector<std::string> lines = parse_input(input_file);

    std::cout << "Part 1: " << solve_part_1(lines) << '\n';
    std::cout << "Part 2: " << solve_part_2(lines) << '\n';

    return 0;
}
```

## Tips

1. **Test First**: Always test with the sample input before running with actual input
   ```bash
   ./run.sh 1 sample
   ```

2. **Reusable Utilities**: Add common parsing or helper functions to `cpp_utils/` for reuse across days

3. **Incremental Development**:
   - Implement and test Part 1 first
   - Then move on to Part 2
   - Use the sample input to validate your logic

4. **Debugging**: Add debug output in your solution code to trace execution
   ```cpp
   std::cout << "Debug: value = " << value << '\n';
   ```

## Compilation Flags

The Makefile uses these flags:
- `-std=c++17` - C++17 standard
- `-Wall -Wextra` - Enable warnings
- `-O2` - Optimization level 2

## Troubleshooting

**"Day X directory does not exist"**
- Run `./new-day.sh X` to create the day directory

**"Input file does not exist"**
- Make sure you've added your input to `<day>/input` or `<day>/sample_1`

**Compilation errors**
- Check that you have a C++17 compatible compiler
- Verify your code syntax in `main.cpp`

## Workflow Example

Complete workflow for solving a new day:

```bash
# 1. Create new day
./new-day.sh 5

# 2. Add inputs (manually copy from Advent of Code website)
# - Copy puzzle input to 5/input
# - Copy sample input to 5/sample_1

# 3. Edit solution
vim 5/main.cpp

# 4. Test with sample
./run.sh 5 sample

# 5. Run with actual input
./run.sh 5
```

## Notes

- Each day is completely independent
- Binaries are named `day_<n>` and placed in their respective directories
- The `cpp_utils` directory contains shared utilities (currently just input parsing helpers)
- Input files are read via stdin (piped to the program)

## License

Personal learning project for Advent of Code 2024.
