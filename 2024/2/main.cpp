#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

std::vector<std::vector<int>> parse_input(std::string input_file) {
    std::ifstream stream{input_file};

    if (!stream) {
        std::cerr << "Input file is not valid\n";
    }

    std::string line;
    std::vector<std::vector<int>> lines;
    while (std::getline(stream, line)) {
        std::vector<int> items;
        std::string item;
        std::istringstream line_stream(line);
        while (std::getline(line_stream, item, ' ')) {
            items.push_back(std::stoi(item));
        }
        lines.push_back(items);
    }

    return lines;
}

bool is_safe_sequence_1(std::vector<int> line) {
    bool safe = true;
    int prev_diff = 0;
    for (int i = 1; i < line.size(); i++) {
        int diff = (line[i] - line[i - 1]);

        std::cout << line[i] << " - " << line[i - 1] << '\n';

        if (diff == 0) {
            std::cout << "0\n";
            safe = false;
            break;
        }
        if ((diff * prev_diff) < 0) {
            std::cout << "-\n";
            safe = false;
            break;
        }
        if (abs(diff) > 3) {
            std::cout << ">\n";
            safe = false;
            break;
        }
        prev_diff = diff;
    }
    return safe;
}

bool is_safe_sequence_2(std::vector<int> line) {
    if (is_safe_sequence_1(line)) return true;

    for (int skip = 0; skip < line.size(); skip++){
        std::vector<int> test_sequence;
        test_sequence.reserve(line.size() - 1);

        for (int i = 0; i < line.size(); i++){
            if (i != skip) test_sequence.push_back(line[i]);
        }

        if (is_safe_sequence_1(test_sequence)) return true;
    }

    return false;
}

int solve_part_1(std::vector<std::vector<int>> lines) {
    int count = 0;
    for (std::vector<int> line : lines) {
        if (is_safe_sequence_1(line)) {
            count++;
            std::cout << "safe\n";
        } else {
            std::cout << "not safe\n";
        }
    }
    return count;
}

int solve_part_2(std::vector<std::vector<int>> lines) {
    int count = 0;
    for (std::vector<int> line : lines) {
        if (is_safe_sequence_2(line)) {
            count++;
            std::cout << "safe\n";
        } else {
            std::cout << "not safe\n";
        }
    }
    return count;
}

int main() {
    std::string input_file;
    std::cin >> input_file;

    std::vector<std::vector<int>> lines = parse_input(input_file);

    std::cout << " -------------------------\n";
    std::cout << " ------SOLVE PART 1-------\n";
    std::cout << " -------------------------\n";
    int part_1 = solve_part_1(lines);

    std::cout << " -------------------------\n";
    std::cout << " ------SOLVE PART 2-------\n";
    std::cout << " -------------------------\n";
    int part_2 = solve_part_2(lines);

    std::cout << part_1 << '\n';
    std::cout << part_2 << '\n';
}
