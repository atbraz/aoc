#include "solution.h"
#include <cassert>
#include <iostream>
#include <print>
#include <string>

int main(int argc, char* argv[]) {
    bool test_mode = (argc > 1 && std::string(argv[1]) == "--test");

    if (test_mode) {
        // TODO: Update expected values after solving with sample input
        std::println("Running tests...\n");

        int result_1 = solve_part_1("sample_1");
        assert(result_1 == 143 && "Part 1 failed");
        std::println("✓ Part 1: {}", result_1);

        int result_2 = solve_part_2("sample_1");
        assert(result_2 == 0 && "Part 2 failed");
        std::println("✓ Part 2: {}", result_2);

        std::println("\n✓ All tests passed!");
        return 0;
    }

    // Normal execution
    std::string input_file;
    std::cin >> input_file;

    std::println("Part 1: {}", solve_part_1(input_file));
    std::println("Part 2: {}", solve_part_2(input_file));

    return 0;
}
