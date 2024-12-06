
#include <array>
#include <cctype>
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

std::vector<std::array<int, 2>> parse_line(std::string line, bool apply_cond) {
    bool enabled = true;
    size_t curr_do_pos = 0;

    size_t curr_pos = 0;
    std::string mul_init = "mul(";
    std::vector<std::array<int, 2>> muls;
    while (true) {

        size_t init_pos = line.find(mul_init, curr_pos);

        if (init_pos == std::string::npos) {
            std::cout << "End of Line\n";
            break;
        } else {
            std::cout << "Found 'mul(' at pos " << init_pos << '\n';
            curr_pos = init_pos;
        }

        size_t do_pos = line.rfind("do()", curr_pos);
        if (do_pos == std::string::npos) {
            do_pos = 0;
        }
        std::cout << "Last 'do()' at pos " << do_pos << '\n';

        size_t dont_pos = line.rfind("don't()", curr_pos);
        std::cout << "Last 'don't() at pos " << dont_pos << '\n';

        if (dont_pos > do_pos) {
            std::cout << "---->Disabled\n";
            continue;
        };

        int curr_number = 0;
        std::array<int, 2> mul;
        while (true) {
            char next_char = line.at(mul_init.length() + ++curr_pos - 1);
            if (!((isdigit(next_char)) | (next_char == ',') |
                  (next_char == ')'))) {
                std::cout << "\t-->Broke out of while on char " << next_char
                          << " at pos" << curr_pos - 1 << '\n';
                break;
            };

            if (isdigit(next_char)) {
                std::cout << "\tFound digit " << next_char << '\n';
                int next_digit = next_char - '0';
                curr_number = (curr_number * 10) + (next_digit);
                continue;
            }

            if (next_char == ',') {
                std::cout << "\tFound number delimiter ',', adding "
                          << curr_number << " to arr\n";
                mul[0] = curr_number;
                curr_number = 0;
                continue;
            }

            if (next_char == ')') {
                std::cout << "\t<--Found mul delimiter ')', pushing mul("
                          << mul[0] << ',' << curr_number << ")' to vec\n";
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
        std::cout << "Parsing line\n\n" << line << "\n\n";
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
    for (std::array<int, 2> mul : muls) {
        result += (mul[0] * mul[1]);
    }
    return result;
}

int main() {
    std::string input_file;
    std::cin >> input_file;

    std::vector<std::string> lines = parse_input(input_file);

    std::cout << " -------------------------\n";
    std::cout << " ------SOLVE PART 1-------\n";
    std::cout << " -------------------------\n";
    int part_1 = mul_vector(get_mul_vectors(lines));

    std::cout << " -------------------------\n";
    std::cout << " ------SOLVE PART 2-------\n";
    std::cout << " -------------------------\n";
    int part_2 = mul_vector(get_mul_vectors(lines, true));

    std::cout << "\nPart 1: " << part_1 << '\n';
    std::cout << "Part 2: " << part_2 << '\n';
}
