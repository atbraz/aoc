#include "solution.h"
#include <algorithm>
#include <cstddef>
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <utility>
#include <vector>

using rule = std::pair<int, int>;
using rules = std::vector<rule>;
using vec_int = std::vector<int>;

int append_digit(int curr_number, int new_digit) { return curr_number * 10 + new_digit; }

rule parse_rules(const std::string &line) {
    int first = 0;
    int second = 0;
    bool passed_pipe = false;
    for (size_t i = 0; i < line.length(); i++) {
        char curr_char = line[i];
        if (line[i] != '|') {
            if (!passed_pipe) {
                first = append_digit(first, curr_char - '0');
            } else {
                second = append_digit(second, curr_char - '0');
            }
        } else {
            passed_pipe = true;
        }
    }

    return std::make_pair(first, second);
}

vec_int parse_pages(const std::string &line) {
    std::vector<int> pages;
    int number = 0;
    size_t line_length = line.length();
    for (size_t i = 0; i < line_length; i++) {
        char curr_char = line[i];
        if (curr_char == ' ') {
            continue;
        }
        if (curr_char == ',' || i == line_length) {
            pages.push_back(number);
            number = 0;
        } else {
            number = append_digit(number, curr_char - '0');
        }
    }
    pages.push_back(number);
    return pages;
}

std::pair<rules, std::vector<vec_int>> parse_input(const std::string &input_file) {
    std::ifstream stream{ input_file };

    if (!stream) {
        std::cerr << "Error: Cannot open input file\n";
        return {};
    }

    std::string line;
    std::vector<std::string> lines;
    rules parsed_rules;
    std::vector<vec_int> parsed_pages;
    while (std::getline(stream, line)) {
        if (line.contains("|")) {
            parsed_rules.push_back(parse_rules(line));
        } else if (line.contains(",")) {
            parsed_pages.push_back(parse_pages(line));
        }
        lines.push_back(line);
    }

    return std::make_pair(parsed_rules, parsed_pages);
}

bool is_ordered(const vec_int &pages, const rules &rules) {
    return std::ranges::all_of(rules, [&](const auto &rule) {
        const auto &[x, y] = rule;
        auto x_pos = std::find(pages.begin(), pages.end(), x);
        auto y_pos = std::find(pages.begin(), pages.end(), y);

        return x_pos == pages.end() || y_pos == pages.end() || x_pos < y_pos;
    });
}

vec_int make_ordered(vec_int pages, const rules &rules) {
    std::sort(pages.begin(), pages.end(), [&](int a, int b) {
        for (const auto &[x, y] : rules) {
            if (x == a && y == b) return true;
            if (x == b && y == a) return false;
        }
        return false;
    });
    return pages;
}

int solve_part_1(const std::string &input_file) {
    const auto lines = parse_input(input_file);
    const rules rules = lines.first;
    const auto pages = lines.second;

    int sum = 0;
    for (const auto &page_sequence : pages) {
        if (is_ordered(page_sequence, rules)) {
            size_t middle_index = page_sequence.size() / 2;
            sum += page_sequence[middle_index];
        }
    }

    return sum;
}

int solve_part_2(const std::string &input_file) {
    const auto lines = parse_input(input_file);
    const rules rules = lines.first;
    const auto pages = lines.second;

    int sum = 0;
    for (auto page_sequence : pages) {
        if (!is_ordered(page_sequence, rules)) {
            page_sequence = make_ordered(page_sequence, rules);
            size_t middle_index = page_sequence.size() / 2;
            sum += page_sequence[middle_index];
        }
    }

    return sum;
}
