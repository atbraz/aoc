#!/usr/bin/env bash
# Generate statistics about the Advent of Code codebase
# Shows lines of code, completion rates, and language metrics

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
GRAY='\033[0;90m'
NC='\033[0m' # No Color

# Function to count lines of code
count_loc() {
    local pattern=$1
    local path=$2

    if [ -d "$path" ]; then
        find "$path" -name "$pattern" -type f -exec wc -l {} + 2>/dev/null | tail -n1 | awk '{print $1}'
    else
        echo "0"
    fi
}

# Function to count solution files
count_solutions() {
    local year=$1
    local count=0

    for day in {1..25}; do
        local solution_file=""
        case $year in
            2025) solution_file="${year}/${day}/main.zig" ;;
            2024) solution_file="${year}/${day}/main.cpp" ;;
            2023) solution_file="${year}/${day}/src/main.rs" ;;
            2022) solution_file="${year}/${day}/main.ml" ;;
        esac

        if [ -f "$solution_file" ]; then
            local lines=$(wc -l < "$solution_file" 2>/dev/null || echo "0")
            if [ "$lines" -gt 20 ]; then
                ((count++))
            fi
        fi
    done

    echo "$count"
}

# Function to calculate completion percentage
calc_percent() {
    local completed=$1
    local total=$2
    echo "scale=1; $completed * 100 / $total" | bc
}

# Change to repo root
cd "$(dirname "$0")/.."

# Header
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}        Advent of Code - Codebase Statistics${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Completion Statistics
echo -e "${CYAN}Completion Status${NC}"
echo -e "${GRAY}────────────────────────────────────────────────────${NC}"

total_completed=0
for year in 2022 2023 2024 2025; do
    if [ -d "$year" ]; then
        completed=$(count_solutions "$year")
        percent=$(calc_percent "$completed" 25)
        total_completed=$((total_completed + completed))

        printf "  %s: %2d/25 stars " "$year" "$completed"
        printf "(${GREEN}%5.1f%%${NC})\n" "$percent"
    fi
done

echo ""
overall_percent=$(calc_percent "$total_completed" 100)
printf "  ${YELLOW}Overall: %2d/100 stars (%.1f%%)${NC}\n" "$total_completed" "$overall_percent"
echo ""

# Lines of Code Statistics
echo -e "${CYAN}Lines of Code${NC}"
echo -e "${GRAY}────────────────────────────────────────────────────${NC}"

# 2022 - OCaml
if [ -d "2022" ]; then
    ml_solutions=$(count_loc "*.ml" "2022/*/")
    ml_common=$(count_loc "*.ml" "2022/common/")
    ml_total=$((ml_solutions + ml_common))
    printf "  ${MAGENTA}OCaml (2022)${NC}\n"
    printf "    Solutions: %6d lines\n" "$ml_solutions"
    printf "    Common:    %6d lines\n" "$ml_common"
    printf "    Total:     ${GREEN}%6d lines${NC}\n" "$ml_total"
    echo ""
fi

# 2023 - Rust
if [ -d "2023" ]; then
    rs_solutions=$(count_loc "*.rs" "2023/*/src/")
    rs_common=$(count_loc "*.rs" "2023/common/src/")
    rs_total=$((rs_solutions + rs_common))
    printf "  ${MAGENTA}Rust (2023)${NC}\n"
    printf "    Solutions: %6d lines\n" "$rs_solutions"
    printf "    Common:    %6d lines\n" "$rs_common"
    printf "    Total:     ${GREEN}%6d lines${NC}\n" "$rs_total"
    echo ""
fi

# 2025 - Zig
zig_total=0
if [ -d "2025" ]; then
    zig_solutions=$(count_loc "*.zig" "2025/*/")
    zig_total=$zig_solutions
    printf "  ${MAGENTA}Zig (2025)${NC}\n"
    printf "    Solutions: %6d lines\n" "$zig_solutions"
    printf "    Total:     ${GREEN}%6d lines${NC}\n" "$zig_total"
    echo ""
fi

# 2024 - C++
if [ -d "2024" ]; then
    cpp_solutions=$(count_loc "*.cpp" "2024/*/")
    cpp_common_src=$(count_loc "*.cpp" "common/cpp/src/")
    cpp_common_hdr=$(count_loc "*.hpp" "common/cpp/include/")
    cpp_common=$((cpp_common_src + cpp_common_hdr))
    cpp_total=$((cpp_solutions + cpp_common))
    printf "  ${MAGENTA}C++ (2024)${NC}\n"
    printf "    Solutions: %6d lines\n" "$cpp_solutions"
    printf "    Common:    %6d lines\n" "$cpp_common"
    printf "    Total:     ${GREEN}%6d lines${NC}\n" "$cpp_total"
    echo ""
fi

# Total lines
total_lines=0
if [ -d "2022" ]; then total_lines=$((total_lines + ml_total)); fi
if [ -d "2023" ]; then total_lines=$((total_lines + rs_total)); fi
if [ -d "2024" ]; then total_lines=$((total_lines + cpp_total)); fi
if [ -d "2025" ]; then total_lines=$((total_lines + zig_total)); fi

printf "  ${YELLOW}All Languages: %6d lines${NC}\n" "$total_lines"
echo ""

# Average lines per solution
if [ "$total_completed" -gt 0 ]; then
    avg_lines=$((total_lines / total_completed))
    printf "  ${GRAY}Average per solution: %d lines${NC}\n" "$avg_lines"
    echo ""
fi

# Language Distribution
echo -e "${CYAN}Language Distribution${NC}"
echo -e "${GRAY}────────────────────────────────────────────────────${NC}"

if [ "$total_lines" -gt 0 ]; then
    if [ -d "2022" ]; then
        ml_percent=$(calc_percent "$ml_total" "$total_lines")
        printf "  OCaml:  %5.1f%%  " "$ml_percent"
        # Draw bar
        filled=$(printf "%.0f" "$(echo "$ml_percent / 2" | bc -l)")
        printf "${MAGENTA}"
        for i in $(seq 1 "$filled" 2>/dev/null || echo ""); do printf "█"; done
        printf "${NC}\n"
    fi

    if [ -d "2023" ]; then
        rs_percent=$(calc_percent "$rs_total" "$total_lines")
        printf "  Rust:   %5.1f%%  " "$rs_percent"
        filled=$(printf "%.0f" "$(echo "$rs_percent / 2" | bc -l)")
        printf "${MAGENTA}"
        for i in $(seq 1 "$filled" 2>/dev/null || echo ""); do printf "█"; done
        printf "${NC}\n"
    fi

    if [ -d "2024" ]; then
        cpp_percent=$(calc_percent "$cpp_total" "$total_lines")
        printf "  C++:    %5.1f%%  " "$cpp_percent"
        filled=$(printf "%.0f" "$(echo "$cpp_percent / 2" | bc -l)")
        printf "${MAGENTA}"
        for i in $(seq 1 "$filled" 2>/dev/null || echo ""); do printf "█"; done
        printf "${NC}\n"
    fi

    if [ -d "2025" ]; then
        zig_percent=$(calc_percent "$zig_total" "$total_lines")
        printf "  Zig:    %5.1f%%  " "$zig_percent"
        filled=$(printf "%.0f" "$(echo "$zig_percent / 2" | bc -l)")
        printf "${MAGENTA}"
        for i in $(seq 1 "$filled" 2>/dev/null || echo ""); do printf "█"; done
        printf "${NC}\n"
    fi
fi
echo ""

# File Statistics
echo -e "${CYAN}File Statistics${NC}"
echo -e "${GRAY}────────────────────────────────────────────────────${NC}"

solution_files=0
if [ -d "2022" ]; then solution_files=$((solution_files + $(find 2022/*/ -name "*.ml" 2>/dev/null | wc -l))); fi
if [ -d "2023" ]; then solution_files=$((solution_files + $(find 2023/*/src/ -name "*.rs" 2>/dev/null | wc -l))); fi
if [ -d "2024" ]; then solution_files=$((solution_files + $(find 2024/*/ -name "*.cpp" 2>/dev/null | wc -l))); fi
if [ -d "2025" ]; then solution_files=$((solution_files + $(find 2025/*/ -name "*.zig" 2>/dev/null | wc -l))); fi

input_files=$(find 202*/*/input -type f 2>/dev/null | wc -l)
sample_files=$(find 202*/*/sample* -type f 2>/dev/null | wc -l)

printf "  Solution files: %3d\n" "$solution_files"
printf "  Input files:    %3d\n" "$input_files"
printf "  Sample files:   %3d\n" "$sample_files"
echo ""

# Repository Statistics
echo -e "${CYAN}Repository Statistics${NC}"
echo -e "${GRAY}────────────────────────────────────────────────────${NC}"

total_files=$(git ls-files | wc -l)
total_commits=$(git rev-list --count HEAD)
repo_size=$(du -sh . 2>/dev/null | awk '{print $1}')

printf "  Total files (tracked): %4d\n" "$total_files"
printf "  Total commits:         %4d\n" "$total_commits"
printf "  Repository size:       %s\n" "$repo_size"
echo ""

# Recent Activity
echo -e "${CYAN}Recent Activity${NC}"
echo -e "${GRAY}────────────────────────────────────────────────────${NC}"

last_commit=$(git log -1 --format="%ar" 2>/dev/null || echo "unknown")
last_commit_msg=$(git log -1 --format="%s" 2>/dev/null || echo "unknown")

printf "  Last commit: %s\n" "$last_commit"
printf "  Message: ${GRAY}%s${NC}\n" "$last_commit_msg"
echo ""

# Most Active Days
echo -e "${CYAN}Most Active Days (by commits)${NC}"
echo -e "${GRAY}────────────────────────────────────────────────────${NC}"

git log --format="%s" | grep -E "day [0-9]+" | \
    sed -E 's/.*day ([0-9]+).*/\1/' | \
    sort | uniq -c | sort -rn | head -5 | \
    while read count day; do
        printf "  Day %2d: %3d commits " "$day" "$count"
        # Draw mini bar
        printf "${BLUE}"
        for i in $(seq 1 $((count / 2))); do printf "▪"; done
        printf "${NC}\n"
    done 2>/dev/null || echo "  ${GRAY}No day-specific commits found${NC}"

echo ""

# Footer
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GRAY}Generated on $(date '+%Y-%m-%d %H:%M:%S')${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
