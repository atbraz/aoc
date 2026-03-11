#include "solution.h"
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

std::vector<std::string> parse_input(const std::string &input_file) {
    std::ifstream stream{ input_file };

    if (!stream) {
        std::cerr << "Error: Cannot open input file\n";
        return {};
    }

    std::string line;
    std::vector<std::string> lines;
    while (std::getline(stream, line)) {
        lines.push_back(line);
    }

    return lines;
}

std::vector<int> generate_representation(const std::string &line) {
    int id = 0;
    std::vector<int> representation;
    for (int i = 0; i < line.size(); i++) {
        if (i % 2 == 0) {
            for (int j = 0; j < line[i] - '0'; j++) {
                representation.push_back(id + '0');
            }
            id++;
        } else {
            for (int j = 0; j < line[i] - '0'; j++) {
                representation.push_back('.');
            }
        }
    }
    return representation;
}

bool is_sparse(const std::vector<int> &representation) {
    bool sparse = false;
    for (int i = 0; i < representation.size() - 1; i++) {
        if (representation[i] == '.' && !(representation[i + 1] == '.')) {
            return true;
        }
    }

    return sparse;
}

std::vector<int> reallocate(const std::vector<int> &representation) {
    std::vector<int> reallocated = representation;

    int to_allocate;
    int first_space;
    for (int i = reallocated.size() - 1; i >= 0; i--) {

        if (reallocated[i] == '.') {
            continue;
        }

        if (!is_sparse(reallocated)) {
            break;
        }

        to_allocate = reallocated[i];
        first_space = std::find(reallocated.begin(), reallocated.end(), '.') - reallocated.begin();
        reallocated[first_space] = to_allocate;
        reallocated[i] = '.';
    }
    return reallocated;
}

std::vector<int> get_chunk_positions(const std::vector<int> &representation, int id) {
    std::vector<int> positions;
    for (int i = 0; i < representation.size(); i++) {
        if (representation[i] == id + '0') {
            positions.push_back(i);
        }
    }

    return positions;
}

std::vector<int> find_free_space(const std::vector<int> &representation, int size) {
    std::vector<int> free_space;

    int i = 0;
    int free_space_count = 0;
    while (free_space_count < size && i < representation.size()) {
        if (free_space_count == size) {
            break;
        }
        if (representation[i] == '.') {
            free_space_count++;
            free_space.push_back(i);
            i++;
        } else {
            free_space_count = 0;
            free_space.clear();
            i++;
        }
    }

    return free_space;
}

std::vector<int> reallocate_chunks(const std::vector<int> &representation) {
    std::vector<int> reallocated = representation;

    int max_id = reallocated[reallocated.size() - 1] - '0';
    for (int i = max_id; i > 0; i--) {

        std::vector<int> positions = get_chunk_positions(reallocated, i);
        std::vector<int> free_space = find_free_space(reallocated, positions.size());
        if (free_space.size() != positions.size()) {
            continue;
        }
        if (free_space[0] >= positions[0]) {
            continue;
        }

        for (int j = 0; j < positions.size(); j++) {
            reallocated[free_space[j]] = i + '0';
            reallocated[positions[j]] = '.';
        }
    }
    return reallocated;
}

long generate_checksum(const std::vector<int> &representation) {
    long checksum = 0;
    for (int i = 0; i < representation.size(); i++) {
        if (representation[i] == '.') {
            continue;
        }

        checksum += (representation[i] - '0') * i;
    }
    return checksum;
}

long solve_part_1(const std::string &input_file) {
    const auto line = parse_input(input_file)[0];
    const auto representation = generate_representation(line);
    const auto reallocated = reallocate(representation);

    return generate_checksum(reallocated);
}

long solve_part_2(const std::string &input_file) {
    const auto line = parse_input(input_file)[0];
    const auto representation = generate_representation(line);
    const auto reallocated = reallocate_chunks(representation);

    return generate_checksum(reallocated);
}
