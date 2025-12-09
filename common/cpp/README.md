# aoc-common-cpp

Shared utilities for Advent of Code solutions in C++.

## Overview

This library provides common functionality to reduce boilerplate in daily puzzle solutions:

- **Input Reading**: Multiple ways to parse input files (lines, grids, paragraphs)
- **String Utilities**: Split, trim, replace, prefix/suffix checking
- **Math Utilities**: GCD, LCM, modular exponentiation

## Installation

This is a header + source library. To use it in your day's solution:

### Option 1: Include in Compilation

```bash
g++ -std=c++17 -I../../common/cpp/include \
    main.cpp ../../common/cpp/src/aoc_common.cpp \
    -o solution
```

### Option 2: Update Justfile

Modify your `2024/justfile` to include the common library:

```just
CXX := "g++"
CXXFLAGS := "-std=c++17 -Wall -Wextra -O2 -I../common/cpp/include"
COMMON_SRC := "../common/cpp/src/aoc_common.cpp"

[group('build')]
build day:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Compiling day {{day}}..."
    {{CXX}} {{CXXFLAGS}} {{day}}/main.cpp {{COMMON_SRC}} -o {{day}}/day_{{day}}
```

## Usage

### Input Reading

```cpp
#include "aoc_common.hpp"
#include <iostream>

using namespace aoc;

int main() {
    // Read as lines
    auto lines = InputReader::as_lines("input");
    for (const auto& line : lines) {
        std::cout << line << '\n';
    }

    // Read as single string
    std::string content = InputReader::as_string("input");

    // Read as character grid
    auto grid = InputReader::as_char_grid("input");
    char cell = grid[row][col];

    // Read as paragraphs (blank line separated)
    auto paragraphs = InputReader::as_paragraphs("input");

    // Read as single line (newlines removed)
    std::string line = InputReader::as_single_line("input");

    return 0;
}
```

#### InputReader Methods

**`as_string(filename)`**
- Reads entire file as single string
- Preserves newlines
- Returns: `std::string`

**`as_lines(filename)`**
- Reads file as vector of lines
- Removes newline characters from each line
- Returns: `std::vector<std::string>`

**`as_char_grid(filename)`**
- Reads file as 2D grid of characters
- Each line becomes a row
- Returns: `std::vector<std::vector<char>>`
- Access: `grid[y][x]`

**`as_paragraphs(filename)`**
- Splits file into groups separated by blank lines
- Returns: `std::vector<std::string>`
- Each paragraph preserves internal newlines

**`as_single_line(filename)`**
- Reads file and removes all newlines
- Returns: `std::string`
- Useful for continuous sequences

### String Utilities

```cpp
#include "aoc_common.hpp"
#include <iostream>

using namespace aoc;

int main() {
    std::string text = "  hello,world,foo  ";

    // Split by delimiter
    auto parts = StringUtils::split(text, ',');
    // ["  hello", "world", "foo  "]

    // Trim whitespace
    auto trimmed = StringUtils::trim(text);
    // "hello,world,foo"

    auto ltrimmed = StringUtils::ltrim(text);
    // "hello,world,foo  "

    auto rtrimmed = StringUtils::rtrim(text);
    // "  hello,world,foo"

    // Check prefix/suffix
    if (StringUtils::starts_with("hello world", "hello")) {
        std::cout << "Starts with hello\n";
    }

    if (StringUtils::ends_with("hello world", "world")) {
        std::cout << "Ends with world\n";
    }

    // Replace all occurrences
    auto replaced = StringUtils::replace_all("foo bar foo", "foo", "baz");
    // "baz bar baz"

    return 0;
}
```

#### StringUtils Methods

**`split(str, delimiter)`**
- Splits string by delimiter character
- Returns: `std::vector<std::string>`

**`trim(str)` / `ltrim(str)` / `rtrim(str)`**
- Remove whitespace from both ends / left / right
- Returns: `std::string`

**`starts_with(str, prefix)` / `ends_with(str, suffix)`**
- Check if string begins/ends with substring
- Returns: `bool`

**`replace_all(str, from, to)`**
- Replace all occurrences of substring
- Returns: `std::string`

### Math Utilities

```cpp
#include "aoc_common.hpp"
#include <iostream>

using namespace aoc;

int main() {
    // Greatest Common Divisor
    int g = Math::gcd(48, 18);  // 6

    // Least Common Multiple
    int l = Math::lcm(12, 15);  // 60

    // Modular Exponentiation: (base^exp) % mod
    long long result = Math::mod_pow(2, 10, 1000);  // 24
    // Efficient for large exponents

    return 0;
}
```

#### Math Methods

**`gcd<T>(a, b)`**
- Greatest common divisor
- Template works with any integer type
- Returns: Same type as input

**`lcm<T>(a, b)`**
- Least common multiple
- Template works with any integer type
- Returns: Same type as input

**`mod_pow(base, exp, mod)`**
- Modular exponentiation: (base^exp) % mod
- Efficient for large exponents
- Returns: `long long`

## Complete Example

```cpp
#include "aoc_common.hpp"
#include <iostream>
#include <string>
#include <vector>

using namespace aoc;

int solve_part_1(const std::vector<std::string>& lines) {
    int sum = 0;
    for (const auto& line : lines) {
        auto parts = StringUtils::split(line, ',');
        for (const auto& part : parts) {
            sum += std::stoi(StringUtils::trim(part));
        }
    }
    return sum;
}

int solve_part_2(const std::vector<std::vector<char>>& grid) {
    int count = 0;
    for (const auto& row : grid) {
        for (char cell : row) {
            if (cell == '#') {
                count++;
            }
        }
    }
    return count;
}

int main() {
    std::string input_file;
    std::cin >> input_file;

    // Part 1: Process lines
    auto lines = InputReader::as_lines(input_file);
    std::cout << "Part 1: " << solve_part_1(lines) << '\n';

    // Part 2: Process grid
    auto grid = InputReader::as_char_grid(input_file);
    std::cout << "Part 2: " << solve_part_2(grid) << '\n';

    return 0;
}
```

Compile and run:

```bash
g++ -std=c++17 -I../../common/cpp/include \
    main.cpp ../../common/cpp/src/aoc_common.cpp \
    -o solution

echo "input" | ./solution
```

## Error Handling

All `InputReader` methods throw `std::runtime_error` if:
- File cannot be opened
- File doesn't exist
- Insufficient permissions

Handle with try-catch:

```cpp
try {
    auto lines = InputReader::as_lines("input");
    // Process lines...
} catch (const std::exception& e) {
    std::cerr << "Error: " << e.what() << '\n';
    return 1;
}
```

## Design Philosophy

- **Header + Source**: Classic C++ library structure
- **Namespace**: Everything in `aoc` namespace to avoid conflicts
- **STL-first**: Uses standard library types and idioms
- **Exception-based**: Uses exceptions for error handling
- **Zero Dependencies**: Only uses C++ standard library

## Adding New Utilities

When you find yourself writing the same code across multiple days:

1. Add declaration to `include/aoc_common.hpp`
2. Add implementation to `src/aoc_common.cpp`
3. Document in this README with examples
4. Test in a solution to verify it works

Common candidates for extraction:
- Grid traversal (neighbors, directions)
- Parsing helpers (integers from strings, coordinates)
- Graph algorithms (BFS, DFS, Dijkstra)
- Data structures (priority queue wrappers, disjoint sets)

## C++17 Features Used

- `std::string_view` (can be added for performance)
- `std::optional` (can be added for safer returns)
- Structured bindings (not used in library, but available in your code)
- `if constexpr` (not used yet, but available)

## Future Enhancements

Consider adding:
- `GridUtils` class for neighbor traversal
- `ParseUtils` for common parsing patterns
- `GraphUtils` for common graph algorithms
- `RangeUtils` for range operations
- Template metaprogramming helpers

## Resources

- [C++ Standard Library Reference](https://en.cppreference.com/)
- [C++ Core Guidelines](https://isocpp.github.io/CppCoreGuidelines/)
