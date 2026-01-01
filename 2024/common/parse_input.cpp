#include <fstream>
#include <print>
#include <string>
#include <string_view>
#include <vector>

std::vector<std::string> parse_input(std::string_view input_file) {
    std::ifstream stream{std::string(input_file)};

    if (!stream) {
        std::println(stderr, "Input file is not valid");
    }

    std::string line;
    std::vector<std::string> lines;
    while (std::getline(stream, line)) {
        lines.push_back(line);
    }

    return lines;
}
