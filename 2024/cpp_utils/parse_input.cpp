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
