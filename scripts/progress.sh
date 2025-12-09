#!/usr/bin/env bash
# Progress tracker for Advent of Code solutions
# Shows visual completion status for each year

set -euo pipefail

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
GRAY='\033[0;90m'
NC='\033[0m' # No Color

# Symbols
STAR="⭐"
EMPTY="·"

# Function to check if a day exists and has a solution file
check_day() {
    local year=$1
    local day=$2
    local dir="${year}/${day}"

    if [ ! -d "$dir" ]; then
        echo "$EMPTY"
        return
    fi

    # Check for solution file based on year
    local solution_file=""
    case $year in
        2024)
            solution_file="${dir}/main.cpp"
            ;;
        2023)
            solution_file="${dir}/src/main.rs"
            ;;
        2022)
            solution_file="${dir}/main.ml"
            ;;
        *)
            echo "$EMPTY"
            return
            ;;
    esac

    if [ -f "$solution_file" ]; then
        # Check if it has actual content (more than just template)
        local lines=$(wc -l < "$solution_file" 2>/dev/null || echo "0")
        if [ "$lines" -gt 20 ]; then
            echo "$STAR"
        else
            echo "${GRAY}${EMPTY}${NC}"
        fi
    else
        echo "$EMPTY"
    fi
}

# Function to count stars for a year
count_stars() {
    local year=$1
    local count=0

    for day in {1..25}; do
        local result=$(check_day "$year" "$day")
        if [[ "$result" == "$STAR" ]]; then
            ((count++))
        fi
    done

    echo "$count"
}

# Function to show progress for a specific year
show_year() {
    local year=$1

    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}Advent of Code $year${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    # Determine language based on year
    local lang=""
    case $year in
        2024) lang="C++" ;;
        2023) lang="Rust" ;;
        2022) lang="OCaml" ;;
    esac

    echo -e "${GRAY}Language: ${lang}${NC}"
    echo ""

    # Show progress in rows of 5 days
    for week in {0..4}; do
        local start=$((week * 5 + 1))
        local end=$((start + 4))

        # Days
        printf "  Days  "
        for day in $(seq $start $end); do
            if [ $day -le 25 ]; then
                printf "%2d  " $day
            fi
        done
        echo ""

        # Stars
        printf "        "
        for day in $(seq $start $end); do
            if [ $day -le 25 ]; then
                local status=$(check_day "$year" "$day")
                if [[ "$status" == "$STAR" ]]; then
                    printf "${GREEN}${STAR}${NC}   "
                else
                    printf "${status}   "
                fi
            fi
        done
        echo ""
    done

    echo ""

    # Count total stars
    local stars=$(count_stars "$year")
    local percent=$((stars * 100 / 25))

    echo -e "  ${YELLOW}Progress: ${stars}/25 stars (${percent}%)${NC}"

    # Progress bar
    local filled=$((stars * 50 / 25))
    local empty=$((50 - filled))
    printf "  ["
    printf "${GREEN}%${filled}s${NC}" | tr ' ' '█'
    printf "%${empty}s" | tr ' ' '░'
    printf "]\n"

    echo ""
}

# Main script
main() {
    local year=${1:-}

    # Change to repo root
    cd "$(dirname "$0")/.."

    if [ -n "$year" ]; then
        # Show specific year
        if [ -d "$year" ]; then
            show_year "$year"
        else
            echo "Error: Year $year not found"
            exit 1
        fi
    else
        # Show all years
        echo ""
        for year in 2022 2023 2024; do
            if [ -d "$year" ]; then
                show_year "$year"
            fi
        done

        # Overall summary
        echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${YELLOW}Overall Summary${NC}"
        echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

        local total=0
        for year in 2022 2023 2024; do
            if [ -d "$year" ]; then
                local stars=$(count_stars "$year")
                total=$((total + stars))
                printf "  %s: %2d/25 stars\n" "$year" "$stars"
            fi
        done

        local max_stars=$((25 * 3))
        local total_percent=$((total * 100 / max_stars))
        echo ""
        echo -e "  ${YELLOW}Total: ${total}/${max_stars} stars (${total_percent}%)${NC}"
        echo ""
    fi
}

main "$@"
