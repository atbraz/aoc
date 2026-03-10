#include "solution.h"
#include <fstream>
#include <iostream>
#include <set>
#include <string>
#include <vector>

using grid_t = std::vector<std::vector<char>>;

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

grid_t parse_grid(const std::vector<std::string> &lines) {
    grid_t grid;
    for (const auto &line : lines) {
        grid.emplace_back(line.size());
        for (size_t i = 0; i < line.size(); ++i) {
            grid.back()[i] = line[i];
        }
    }
    return grid;
}

std::vector<char> get_antennas(const grid_t &grid) {
    std::vector<char> antennas;
    for (const auto &row : grid) {
        for (const auto &cell : row) {
            if (cell != '.' &&
                std::find(antennas.begin(), antennas.end(), cell) == antennas.end()) {
                antennas.push_back(cell);
            }
        }
    }
    return antennas;
}

grid_t filter_grid(const grid_t &grid, const char &filter) {
    grid_t filtered_grid;
    for (const auto &row : grid) {
        filtered_grid.emplace_back(row.size());
        for (size_t i = 0; i < row.size(); ++i) {
            filtered_grid.back()[i] = row[i] == filter ? filter : '.';
        }
    }
    return filtered_grid;
}

std::vector<std::pair<int, int>> get_antenna_positions(const grid_t &grid) {
    std::vector<std::pair<int, int>> antenna_positions;
    for (size_t i = 0; i < grid.size(); ++i)
        for (size_t j = 0; j < grid[i].size(); ++j) {
            if (grid[i][j] == '.') {
                continue;
            }
            antenna_positions.emplace_back(i, j);
        }
    return antenna_positions;
}

void collect_filtered_antinodes(
    const grid_t &filtered_grid,
    std::set<std::pair<int, int>> &antinode_positions
) {
    std::vector<std::pair<int, int>> antenna_positions = get_antenna_positions(filtered_grid);

    int rows = static_cast<int>(filtered_grid.size());
    int cols = static_cast<int>(filtered_grid[0].size());
    for (size_t i = 0; i < antenna_positions.size(); ++i) {
        for (size_t j = 0; j < antenna_positions.size(); ++j) {
            if (i == j) {
                continue;
            }

            int dx = antenna_positions[i].first - antenna_positions[j].first;
            int dy = antenna_positions[i].second - antenna_positions[j].second;

            int ar = antenna_positions[i].first + dx;
            int ac = antenna_positions[i].second + dy;

            if (ar >= 0 && ar < rows && ac >= 0 && ac < cols) {
                antinode_positions.emplace(ar, ac);
            }
        }
    }
}

void collect_filtered_antinodes_2(
    const grid_t &filtered_grid,
    std::set<std::pair<int, int>> &antinode_positions
) {
    std::vector<std::pair<int, int>> antenna_positions = get_antenna_positions(filtered_grid);

    int rows = static_cast<int>(filtered_grid.size());
    int cols = static_cast<int>(filtered_grid[0].size());
    for (size_t i = 0; i < antenna_positions.size(); ++i) {
        for (size_t j = 0; j < antenna_positions.size(); ++j) {
            if (i == j) {
                continue;
            }

            int dx, dy, ar, ac;

            int dmult = 0;
            do {
                dx = antenna_positions[i].first - antenna_positions[j].first;
                dy = antenna_positions[i].second - antenna_positions[j].second;

                ar = antenna_positions[i].first + dmult * dx;
                ac = antenna_positions[i].second + dmult * dy;
                if (ar >= 0 && ar < rows && ac >= 0 && ac < cols) {
                    antinode_positions.emplace(ar, ac);
                    ++dmult;
                }
            } while (ar >= 0 && ar < rows && ac >= 0 && ac < cols);
        }
    }
}

int solve_part_1(const std::string &input_file) {

    const auto lines = parse_input(input_file);
    const auto grid = parse_grid(lines);
    const auto antennas = get_antennas(grid);

    std::set<std::pair<int, int>> antinode_positions;
    for (const auto &filter : antennas) {
        const auto filtered_grid = filter_grid(grid, filter);
        collect_filtered_antinodes(filtered_grid, antinode_positions);
    }

    return static_cast<int>(antinode_positions.size());
}

int solve_part_2(const std::string &input_file) {
    const auto lines = parse_input(input_file);
    const auto grid = parse_grid(lines);
    const auto antennas = get_antennas(grid);

    std::set<std::pair<int, int>> antinode_positions;
    for (const auto &filter : antennas) {
        const auto filtered_grid = filter_grid(grid, filter);
        collect_filtered_antinodes_2(filtered_grid, antinode_positions);
    }

    return static_cast<int>(antinode_positions.size());
}
