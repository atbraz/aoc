#!/bin/bash

# Quick run script for Advent of Code solutions
# Usage: ./run.sh <day> [sample|input]
# Examples:
#   ./run.sh 1          - Run day 1 with input file
#   ./run.sh 1 sample   - Run day 1 with sample file
#   ./run.sh 1 input    - Run day 1 with input file

set -e

# Check if day number is provided
if [ -z "$1" ]; then
    echo "Usage: ./run.sh <day> [sample|input]"
    echo ""
    echo "Examples:"
    echo "  ./run.sh 1          - Run day 1 with input file"
    echo "  ./run.sh 1 sample   - Run day 1 with sample file"
    echo "  ./run.sh 1 input    - Run day 1 with input file"
    exit 1
fi

DAY=$1
MODE=${2:-input}  # Default to 'input' if not specified

# Check if day directory exists
if [ ! -d "$DAY" ]; then
    echo "Error: Day $DAY directory does not exist!"
    echo "Create it with: ./new-day.sh $DAY"
    exit 1
fi

# Determine input file
if [ "$MODE" = "sample" ]; then
    INPUT_FILE="$DAY/sample_1"
    echo "Running day $DAY with sample input..."
else
    INPUT_FILE="$DAY/input"
    echo "Running day $DAY with input file..."
fi

# Check if input file exists
if [ ! -f "$INPUT_FILE" ]; then
    echo "Error: Input file $INPUT_FILE does not exist!"
    exit 1
fi

# Compile
echo "Compiling..."
make $DAY --no-print-directory

# Run
echo ""
echo "$INPUT_FILE" | ./$DAY/day_$DAY
