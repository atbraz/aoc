#include "solution.h"
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

using equation = std::pair<long, std::vector<long>>;
std::vector<std::string> k_operators = { "+", "*", "||" };

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

std::vector<equation> parse_lines(const std::vector<std::string> &lines) {
    std::vector<equation> parsed_lines;

    for (const auto &line : lines) {
        std::stringstream stream{ line };
        std::string token;

        std::getline(stream, token, ':');
        long result = std::stol(token);

        std::vector<long> tokens;
        long operand;
        while (stream >> operand) {
            tokens.push_back(operand);
        }
        parsed_lines.emplace_back(result, std::move(tokens));
    }

    return parsed_lines;
}

std::vector<std::vector<std::string>> cartesian_operations(const equation &equation) {

    std::vector<std::vector<std::string>> possible_operations;
    int n = equation.second.size() - 1;
    int k = k_operators.size();
    std::vector<int> indices(n);

    while (true) {
        std::vector<std::string> combo(n);
        for (int i = 0; i < n; i++) {
            combo[i] = k_operators[indices[i]];
        }
        possible_operations.push_back(combo);

        int index = n - 1;
        while (index >= 0 && ++indices[index] == k) {
            indices[index--] = 0;
        }

        if (index < 0) {
            break;
        }
    }
    return possible_operations;
}

long compute_result(
    const std::vector<long> &operands,
    const std::vector<std::string> &operators,
    bool with_concatenation
) {
    long result = operands[0];

    for (int i = 0; i < operators.size(); i++) {

        const std::string &op = operators[i];
        const long &operand = operands[i + 1];

        if (op == "+") {
            result += operand;
        } else if (op == "*") {
            result *= operand;

        } else if (op == "||" && with_concatenation) {
            result = std::stol(std::to_string(result) + std::to_string(operand));
        }
    }
    return result;
}


long sum_valid_results(const std::vector<equation> &parsed_lines, bool with_concatenation) {
    long valid_results = 0;

    for (const auto &[result, operands] : parsed_lines) {
        const auto possible_operations = cartesian_operations({ result, operands });
        for (auto &op : possible_operations) {
            if (compute_result(operands, op, with_concatenation) == result) {
                valid_results += result;
                break;
            }
        }
    }

    return valid_results;
}

long solve_part_1(const std::string &input_file) {
    const auto lines = parse_input(input_file);
    const auto parsed_lines = parse_lines(lines);
    return sum_valid_results(parsed_lines, false);
}

long solve_part_2(const std::string &input_file) {
    const auto lines = parse_input(input_file);
    const auto parsed_lines = parse_lines(lines);
    return sum_valid_results(parsed_lines, true);
}
