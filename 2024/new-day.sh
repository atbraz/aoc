#!/bin/bash

# Script to create a new Advent of Code solution day
# Usage: ./new-day.sh <day_number>

set -e

# Check if day number is provided
if [ -z "$1" ]; then
    echo "Usage: ./new-day.sh <day_number>"
    echo "Example: ./new-day.sh 5"
    exit 1
fi

DAY=$1

# Check if day already exists
if [ -d "$DAY" ]; then
    echo "Error: Day $DAY already exists!"
    exit 1
fi

# Create directory
echo "Creating directory for day $DAY..."
mkdir -p "$DAY"

# Create main.cpp template
cat > "$DAY/main.cpp" << 'EOF'
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

std::vector<std::string> parse_input(std::string input_file) {
    std::ifstream stream{input_file};

    if (!stream) {
        std::cerr << "Error: Cannot open input file\n";
        return {};
    }

    std::string line;
    std::vector<std::string> lines;
    while (std::getline(stream, line)) {
        lines.push_back(line);
    }

    return lines;
}

int solve_part_1(const std::vector<std::string>& lines) {
    // TODO: Implement part 1 solution
    return 0;
}

int solve_part_2(const std::vector<std::string>& lines) {
    // TODO: Implement part 2 solution
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
EOF

# Create empty input files
echo "Creating input files..."
touch "$DAY/input"
touch "$DAY/sample_1"

# Create Makefile
echo "Creating Makefile..."
cat > "$DAY/Makefile" << EOF
# Makefile for Day $DAY
# Usage:
#   make        - Compile the solution
#   make run    - Compile and run with input file
#   make test   - Compile and run with sample input
#   make clean  - Remove compiled binary

CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -O2
DAY = $DAY
BINARY = day_\$(DAY)

# Colors for output
GREEN = \\033[0;32m
YELLOW = \\033[0;33m
NC = \\033[0m # No Color

.PHONY: all run test clean

# Default target - compile
all: \$(BINARY)

\$(BINARY): main.cpp
	@echo "\$(GREEN)Compiling day \$(DAY)...\$(NC)"
	@\$(CXX) \$(CXXFLAGS) main.cpp -o \$(BINARY)

# Run with input file
run: \$(BINARY)
	@echo "\$(GREEN)Running day \$(DAY) with input...\$(NC)"
	@echo "input" | ./\$(BINARY)

# Run with sample file
test: \$(BINARY)
	@echo "\$(GREEN)Running day \$(DAY) with sample input...\$(NC)"
	@echo "sample_1" | ./\$(BINARY)

# Clean compiled binary
clean:
	@echo "\$(YELLOW)Cleaning day \$(DAY)...\$(NC)"
	@rm -f \$(BINARY)
EOF

echo ""
echo "✓ Successfully created day $DAY!"
echo ""
echo "Next steps:"
echo "  1. Add your puzzle input to $DAY/input"
echo "  2. Add sample input to $DAY/sample_1"
echo "  3. Implement your solution in $DAY/main.cpp"
echo "  4. Test with: cd $DAY && make test"
echo "  5. Run with: cd $DAY && make run"
