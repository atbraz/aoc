#include "../cpp_utils/parse_input.h"
#include <array>
#include <iostream>
#include <print>
#include <ranges>
#include <string>
#include <vector>

typedef std::vector<std::vector<char>> char_matrix;

int count_xmas(const char_matrix &chars) {
    int count = 0;
    const std::string target = "XMAS";

    const std::array<std::pair<int, int>, 8> directions = {
        { { -1, 0 }, { -1, 1 }, { 0, 1 }, { 1, 1 }, { 1, 0 }, { 1, -1 }, { 0, -1 }, { -1, -1 } }
    };

    for (int i = 0; i < static_cast<int>(chars.size()); i++) {
        for (int j = 0; j < static_cast<int>(chars[i].size()); j++) {
            for (const auto &[di, dj] : directions) {
                bool found = true;
                for (int k = 0; k < 4; k++) {
                    int ni = i + k * di;
                    int nj = j + k * dj;
                    if (ni < 0 || ni >= static_cast<int>(chars.size()) || nj < 0 ||
                        nj >= static_cast<int>(chars[0].size()) || chars[ni][nj] != target[k]) {
                        found = false;
                        break;
                    }
                }
                if (found)
                    count++;
            }
        }
    }

    return count;
}

int count_x_mas(const char_matrix &chars) {
    int count = 0;

    for (int i = 1; i < static_cast<int>(chars.size()) - 1; i++) {
        for (int j = 1; j < static_cast<int>(chars[i].size()) - 1; j++) {
            if (chars[i][j] != 'A') {
                continue;
            }

            bool diag1_mas = (chars[i - 1][j - 1] == 'M' && chars[i + 1][j + 1] == 'S');
            bool diag1_sam = (chars[i - 1][j - 1] == 'S' && chars[i + 1][j + 1] == 'M');
            bool diag2_mas = (chars[i - 1][j + 1] == 'M' && chars[i + 1][j - 1] == 'S');
            bool diag2_sam = (chars[i - 1][j + 1] == 'S' && chars[i + 1][j - 1] == 'M');

            if ((diag1_mas || diag1_sam) && (diag2_mas || diag2_sam)) {
                count++;
            }
        }
    }

    return count;
}

int main() {
    std::string input_file;
    std::cin >> input_file;

    const auto lines = parse_input(input_file);
    char_matrix chars;

    for (const auto &line : lines) {
        chars.emplace_back(line.begin(), line.end());
    }

    std::println("PART 1");
    std::println("{}\n", count_xmas(chars));

    std::println("PART 2");
    std::println("{}\n", count_x_mas(chars));
}
