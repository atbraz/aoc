#include "solution.h"
#include <fstream>
#include <iostream>
#include <set>
#include <string>
#include <utility>
#include <vector>

const char k_dir = '^';
const char k_directions[] = { '^', 'v', '<', '>' };

std::vector<std::string> parse_input(const std::string &input_file) {
    std::ifstream stream{ input_file };

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

std::pair<int, int> find_start(const std::vector<std::string> &lines) {
    int rows = lines.size();
    int cols = lines[0].size();
    int row = 0, col = 0;
    auto dir = k_dir;

    for (auto r = 0; r < rows; r++) {
        for (const auto &d : k_directions) {
            auto pos = lines[r].find(k_dir);
            if (pos != std::string::npos) {
                row = r;
                col = pos;
            }
        }
    }

    return std::make_pair(row, col);
}

int next_row(char d) { return d == '^' ? -1 : d == 'v' ? 1 : 0; }
int next_col(char d) { return d == '<' ? -1 : d == '>' ? 1 : 0; }
char turn_right(char d) {
    switch (d) {
    case '^':
        return '>';
    case '>':
        return 'v';
    case 'v':
        return '<';
    case '<':
        return '^';
    }
    return d;
}

bool exits(const std::vector<std::string> &lines) {
    int rows = lines.size();
    int cols = lines[0].size();
    auto dir = k_dir;
    auto [row, col] = find_start(lines);

    std::vector<std::vector<std::set<char>>> visited_directions =
        std::vector<std::vector<std::set<char>>>(rows, std::vector<std::set<char>>(cols));
    visited_directions[row][col].insert(dir);

    while (row >= 0 && row < rows && col >= 0 && col < cols) {
        int nr = row + next_row(dir);
        int nc = col + next_col(dir);

        if (nr < 0 || nr >= rows || nc < 0 || nc >= cols) {
            return true;
        }

        if (lines[nr][nc] == '#') {
            dir = turn_right(dir);
        } else {

            if (visited_directions[nr][nc].find(dir) != visited_directions[nr][nc].end()) {
                return false;
            }
            visited_directions[nr][nc].insert(dir);

            row = nr;
            col = nc;
        }
    }
}

int calc_traversed(const std::vector<std::string> &lines) {

    int rows = lines.size();
    int cols = lines[0].size();
    auto dir = k_dir;
    auto grid = lines;
    auto [row, col] = find_start(lines);

    while (row >= 0 && row < rows && col >= 0 && col < cols) {
        grid[row][col] = 'X';
        int nr = row + next_row(dir);
        int nc = col + next_col(dir);

        if (nr < 0 || nr >= rows || nc < 0 || nc >= cols)
            break;

        if (lines[nr][nc] == '#') {
            dir = turn_right(dir);
        } else {
            row = nr;
            col = nc;
        }
    }

    int visited = 0;
    for (const auto &line : grid) {
        for (char c : line) {
            if (c == 'X')
                visited++;
        }
    }
    return visited;
}

int calc_obstacles(const std::vector<std::string> &lines) {

    int rows = lines.size();
    int cols = lines[0].size();
    auto dir = k_dir;
    auto [row, col] = find_start(lines);

    auto loops = 0;

    for (auto r = 0; r < rows; r++) {
        for (auto c = 0; c < cols; c++) {
            if (lines[r][c] != '#' && lines[r][c] != k_dir && !(r == row && c == col)) {

                auto grid = lines;

                grid[r][c] = '#';
                auto this_exists = exits(grid);
                if (this_exists) {
                } else {
                    loops++;
                }
            }
        }
    }

    return loops;
}

int solve_part_1(const std::string &input_file) {
    const auto lines = parse_input(input_file);
    return calc_traversed(lines);
}

int solve_part_2(const std::string &input_file) {
    const auto lines = parse_input(input_file);
    return calc_obstacles(lines);
}
