
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

std::vector<std::array<int, 2>> parse_lines(std::string line) {
    int curr_pos = 0;
    int curr_digit;
    bool matching = false;
    std::string mul_init = "mul(";
    std::vector<std::array<int, 2>> muls;
    while (true){
        size_t init_pos = line.find(mul_init);
        if (init_pos == std::string::npos) break;
        // TODO: draw the rest of the fucking owl
        int next_digit = line.at(init_pos + mul_init.length());
        if (isdigit(next_digit)){
            curr_digit = (curr_digit * 10) + next_digit;
            matching = true;
        }
    }
}

int main() {
    std::string input_file;
    std::cin >> input_file;

    std::vector<std::string> lines = parse_input(input_file);

    std::cout << " -------------------------\n";
    std::cout << " ------SOLVE PART 1-------\n";
    std::cout << " -------------------------\n";
    int part_1 = 0;

    std::cout << " -------------------------\n";
    std::cout << " ------SOLVE PART 2-------\n";
    std::cout << " -------------------------\n";
    int part_2 = 0;

    std::cout << part_1 << '\n';
    std::cout << part_2 << '\n';
}
