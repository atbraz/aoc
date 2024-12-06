#include "../cpp_utils/parse_input.h"
#include <iostream>

int main() {

    std::string input_file;
    std::cin >> input_file;

    std::vector<std::string> lines = parse_input(input_file);
    for (std::string line : lines) {
        std::cout << line << '\n';
    }
}
