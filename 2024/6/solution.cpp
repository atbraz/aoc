#include "solution.h"
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

std::vector<std::string> parse_input(const std::string& input_file) {
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

int solve_part_1(const std::string& input_file) {
    const auto lines = parse_input(input_file);
    // TODO: Implement part 1 solution
    return 0;
}

int solve_part_2(const std::string& input_file) {
    const auto lines = parse_input(input_file);
    // TODO: Implement part 2 solution
    return 0;
}
