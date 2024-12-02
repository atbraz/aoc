#include <algorithm>
#include <cstddef>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

int solve_part_1(std::ifstream &stream) {

    std::string line;
    std::vector<int> first_list, second_list;
    while (std::getline(stream, line)) {
        size_t tab_pos = line.find("   ");
        first_list.push_back(std::stoi(line.substr(0, tab_pos)));
        second_list.push_back(std::stoi(line.substr(tab_pos + 1)));
    }

    if (first_list.size() != second_list.size()) {
        std::cerr << "Lists are not same size\n";
        return 1;
    }
    std::sort(first_list.begin(), first_list.end());
    std::sort(second_list.begin(), second_list.end());

    int cum_diff = 0;
    for (int i = 0; i < first_list.size(); i++) {
        cum_diff += abs(first_list[i] - second_list[i]);
    }
    return cum_diff;
}

int solve_part_2(std::ifstream &stream) {

    std::string line;
    std::unordered_map<int, int> count;
    while (std::getline(stream, line)) {
        size_t tab_pos = line.find("   ");
        int first_value = std::stoi(line.substr(0, tab_pos));
        int second_value = std::stoi(line.substr(tab_pos + 1));

        std::cout << first_value << '\t'<< second_value << '\n';

        if (count.find(first_value) == count.end()) {
            count[first_value] = 0;
        }

        if (count.find(second_value) == count.end()) {
            count[second_value] = 1;
        } else {
            count[second_value] += 1;
        }
    }

    std::cout << "---------" << '\n';

    int cum_sum;
    for (const auto &[key, value] : count) {
        std::cout << key << '\t' << value << '\n';
        cum_sum += key * value;
        std::cout << cum_sum << '\n';
    }

    return cum_sum;
}

int main() {
    std::string input_file;
    std::cin >> input_file;
    std::ifstream stream{input_file};

    if (!stream) {
        std::cerr << "Error\n";
        return 1;
    }

    std::cout << "Part 1: " << solve_part_1(stream) << '\n';
    stream.clear();
    stream.seekg(0);
    std::cout << "Part 2: " << solve_part_2(stream) << '\n';

    return 0;
}
