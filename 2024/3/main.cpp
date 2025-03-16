
#include <array>
#include <cctype>
#include <cstddef>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

std::vector<std::string> parse_input(std::string input_file) {
    std::ifstream stream{input_file};

    if (!stream) {
        std::cerr << "Input file is not valid\n";
    }

    std::string line;
    std::vector<std::string> lines;
    while (std::getline(stream, line)) {
        lines.push_back(line);
    }

    return lines;
}

std::string do_dont_pos(size_t do_pos, size_t dont_pos) {
    std::string do_str =
        do_pos == std::string::npos ? "-" : std::to_string(do_pos);
    std::string dont_str =
        dont_pos == std::string::npos ? "-" : std::to_string(dont_pos);
    return "\t\t| do() @ " + do_str + "\t| don't() @ " + dont_str + "\n";
}

std::vector<std::array<int, 2>> parse_line(std::string line, bool apply_cond) {
    bool enabled = true;
    size_t curr_pos = 0;
    std::string mul_init = "mul(";
    std::vector<std::array<int, 2>> muls;
    size_t last_control = std::string::npos;

    while (curr_pos < line.length()) {

        size_t init_pos = line.find(mul_init, curr_pos);

        if (init_pos == std::string::npos) {
            std::cout
                << "\n\t------------------End of Line------------------\n";
            break;
        } else {
            curr_pos = init_pos;
        }

        size_t do_pos = line.rfind("do()", curr_pos);
        size_t dont_pos = line.rfind("don't()", curr_pos);

        std::cout << curr_pos << "\t| mul(";
        if (apply_cond) {
            size_t last_do = std::string::npos;
            size_t last_dont = std::string::npos;

            if (do_pos < curr_pos) {
                last_do = do_pos;
            }
            if (dont_pos < curr_pos) {
                last_dont = dont_pos;
            }

            if (last_do != std::string::npos ||
                last_dont != std::string::npos) {
                enabled =
                    (!(last_dont != std::string::npos &&
                       (last_do == std::string::npos || last_dont > last_do)));
            }
        }

        if (!enabled) {
            std::cout << "\t\t--> Disabled\t" << do_dont_pos(do_pos, dont_pos);
            curr_pos++;
            continue;
        };

        int curr_number = 0;
        std::array<int, 2> mul;
        while (curr_pos < line.length()) {
            char next_char = line.at(mul_init.length() + ++curr_pos - 1);
            if (!((isdigit(next_char)) | (next_char == ',') |
                  (next_char == ')'))) {
                std::cout << next_char << "\t--> Broken\t"
                          << do_dont_pos(do_pos, dont_pos);
                break;
            };

            if (isdigit(next_char)) {
                std::cout << next_char;
                int next_digit = next_char - '0';
                curr_number = (curr_number * 10) + (next_digit);
                continue;
            }

            if (next_char == ',') {
                std::cout << ',';
                mul[0] = curr_number;
                curr_number = 0;
                continue;
            }

            if (next_char == ')') {
                std::cout << ")\t<-- pushing to vec"
                          << do_dont_pos(do_pos, dont_pos);
                mul[1] = curr_number;
                muls.push_back(mul);
                break;
            }
        }
    }

    return muls;
}

std::vector<std::array<int, 2>> get_mul_vectors(std::vector<std::string> lines,
                                                bool apply_cond = false) {
    std::vector<std::array<int, 2>> result;

    for (std::string line : lines) {
        std::cout << "\n\nParsing line\n\n" << line << "\n\n";
        std::vector<std::array<int, 2>> parsed_line =
            parse_line(line, apply_cond);
        for (std::array<int, 2> arr : parsed_line) {
            result.push_back(arr);
        }
    }

    return result;
}

int mul_vector(std::vector<std::array<int, 2>> muls) {
    int result = 0;
    std::cout << "\nMultiplications being summed:\n";
    for (std::array<int, 2> mul : muls) {
        result += (mul[0] * mul[1]);
        std::cout << mul[0] << "\t*\t" << mul[1] << "\t=\t" << (mul[0] * mul[1])
                  << "\t| Running total: " << result << "\n";
    }
    return result;
}

int main() {
    std::string input_file;
    std::cin >> input_file;

    std::vector<std::string> lines = parse_input(input_file);

    std::cout << "\n\t-------------------------\n";
    std::cout << "\t------SOLVE PART 1-------\n";
    std::cout << "\t-------------------------\n";
    int part_1 = mul_vector(get_mul_vectors(lines));

    std::cout << "\n\t-------------------------\n";
    std::cout << "\t------SOLVE PART 2-------\n";
    std::cout << "\t-------------------------\n";
    int part_2 = mul_vector(get_mul_vectors(lines, true));

    std::cout << "\nPart 1: " << part_1 << '\n';
    std::cout << "Part 2: " << part_2 << '\n';
}
