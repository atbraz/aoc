#!/usr/bin/env bash
# Fetch Advent of Code puzzle input using session cookie
# Requires: curl
# Usage: ./fetch-input.sh <year> <day>

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SESSION_FILE="${AOC_SESSION_FILE:-$HOME/.config/aoc-session}"
AOC_URL="https://adventofcode.com"

# Function to print colored messages
error() {
    echo -e "${RED}Error:${NC} $1" >&2
}

success() {
    echo -e "${GREEN}✓${NC} $1"
}

info() {
    echo -e "${BLUE}→${NC} $1"
}

warn() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Function to check if session cookie exists
check_session() {
    if [ ! -f "$SESSION_FILE" ]; then
        error "Session file not found: $SESSION_FILE"
        echo ""
        echo "To use this script, you need to provide your AOC session cookie:"
        echo ""
        echo "1. Log into https://adventofcode.com in your browser"
        echo "2. Open browser DevTools (F12)"
        echo "3. Go to Application/Storage > Cookies > https://adventofcode.com"
        echo "4. Copy the value of the 'session' cookie"
        echo "5. Create the session file:"
        echo ""
        echo "   mkdir -p ~/.config"
        echo "   echo 'your-session-cookie-value' > ~/.config/aoc-session"
        echo "   chmod 600 ~/.config/aoc-session"
        echo ""
        echo "Alternatively, set the AOC_SESSION_FILE environment variable:"
        echo "   export AOC_SESSION_FILE=/path/to/your/session-file"
        echo ""
        exit 1
    fi

    if [ ! -r "$SESSION_FILE" ]; then
        error "Session file is not readable: $SESSION_FILE"
        exit 1
    fi
}

# Function to validate year
validate_year() {
    local year=$1

    if ! [[ "$year" =~ ^[0-9]{4}$ ]]; then
        error "Invalid year format: $year"
        exit 1
    fi

    if [ "$year" -lt 2015 ] || [ "$year" -gt $(date +%Y) ]; then
        error "Year must be between 2015 and $(date +%Y)"
        exit 1
    fi
}

# Function to validate day
validate_day() {
    local day=$1

    if ! [[ "$day" =~ ^[0-9]+$ ]]; then
        error "Invalid day format: $day"
        exit 1
    fi

    if [ "$day" -lt 1 ] || [ "$day" -gt 25 ]; then
        error "Day must be between 1 and 25"
        exit 1
    fi
}

# Function to fetch input
fetch_input() {
    local year=$1
    local day=$2
    local output_file=$3

    local session=$(cat "$SESSION_FILE" | tr -d '\n\r ')
    local url="${AOC_URL}/${year}/day/${day}/input"

    info "Fetching input for ${year} day ${day}..."

    local response=$(curl -s -w "\n%{http_code}" \
        -H "Cookie: session=${session}" \
        -H "User-Agent: github.com/yourusername/aoc fetch script" \
        "$url")

    local http_code=$(echo "$response" | tail -n1)
    local content=$(echo "$response" | head -n1)

    case $http_code in
        200)
            echo -n "$content" > "$output_file"
            success "Input saved to: $output_file"
            local size=$(wc -c < "$output_file")
            info "File size: ${size} bytes"
            return 0
            ;;
        400)
            error "Bad request - check your year and day"
            return 1
            ;;
        404)
            error "Not found - puzzle may not be released yet"
            info "Puzzles unlock at midnight EST (UTC-5)"
            return 1
            ;;
        500)
            error "Server error - try again later"
            return 1
            ;;
        *)
            error "Unexpected response code: $http_code"
            if [ -n "$content" ]; then
                echo "Response: $content"
            fi
            return 1
            ;;
    esac
}

# Main script
main() {
    if [ $# -lt 2 ]; then
        echo "Usage: $0 <year> <day>"
        echo ""
        echo "Example:"
        echo "  $0 2024 5    # Fetch input for 2024 day 5"
        echo ""
        echo "The input will be saved to <year>/<day>/input"
        echo ""
        exit 1
    fi

    local year=$1
    local day=$2

    # Validate inputs
    validate_year "$year"
    validate_day "$day"

    # Check session cookie
    check_session

    # Change to repo root
    cd "$(dirname "$0")/.."

    # Check if year directory exists
    if [ ! -d "$year" ]; then
        error "Year directory not found: $year"
        info "Create it first with: just new $year $day"
        exit 1
    fi

    # Create day directory if it doesn't exist
    local day_dir="${year}/${day}"
    if [ ! -d "$day_dir" ]; then
        warn "Day directory not found: $day_dir"
        read -p "Create it? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            info "Creating day directory..."
            just new "$year" "$day"
        else
            error "Aborted"
            exit 1
        fi
    fi

    # Check if input already exists
    local output_file="${day_dir}/input"
    if [ -f "$output_file" ]; then
        warn "Input file already exists: $output_file"
        local size=$(wc -c < "$output_file")
        if [ "$size" -gt 0 ]; then
            warn "File is not empty (${size} bytes)"
            read -p "Overwrite? (y/n) " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                info "Aborted"
                exit 0
            fi
        fi
    fi

    # Fetch the input
    if fetch_input "$year" "$day" "$output_file"; then
        echo ""
        success "Ready to solve! Run: just test $year $day"
    else
        exit 1
    fi
}

main "$@"
