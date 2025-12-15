#include "../cpp_utils/parse_input.h"
#include <iostream>
#include <print>
#include <string>

int main() {
    std::string input_file;
    std::cin >> input_file;

    const auto lines = parse_input(input_file);
    for (const auto& line : lines) {
        std::println("{}", line);
    }
}
