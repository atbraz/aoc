#pragma once

#include <string>
#include <vector>

namespace aoc {

/**
 * Input reading utilities for Advent of Code
 */
class InputReader {
public:
    /**
     * Read entire file as a single string
     * @param filename Path to input file
     * @return File contents as string
     */
    static std::string as_string(const std::string& filename);

    /**
     * Read file as vector of lines
     * @param filename Path to input file
     * @return Vector of lines (without newline characters)
     */
    static std::vector<std::string> as_lines(const std::string& filename);

    /**
     * Read file as 2D character grid
     * @param filename Path to input file
     * @return 2D vector of characters
     */
    static std::vector<std::vector<char>> as_char_grid(const std::string& filename);

    /**
     * Read file and split into paragraphs (groups separated by blank lines)
     * @param filename Path to input file
     * @return Vector of paragraph strings
     */
    static std::vector<std::string> as_paragraphs(const std::string& filename);

    /**
     * Read file as single line (all newlines removed)
     * @param filename Path to input file
     * @return Single continuous string
     */
    static std::string as_single_line(const std::string& filename);
};

/**
 * String utilities
 */
class StringUtils {
public:
    /**
     * Split string by delimiter
     * @param str String to split
     * @param delimiter Delimiter character
     * @return Vector of substrings
     */
    static std::vector<std::string> split(const std::string& str, char delimiter);

    /**
     * Trim whitespace from both ends
     * @param str String to trim
     * @return Trimmed string
     */
    static std::string trim(const std::string& str);

    /**
     * Trim whitespace from left
     * @param str String to trim
     * @return Left-trimmed string
     */
    static std::string ltrim(const std::string& str);

    /**
     * Trim whitespace from right
     * @param str String to trim
     * @return Right-trimmed string
     */
    static std::string rtrim(const std::string& str);

    /**
     * Check if string starts with prefix
     * @param str String to check
     * @param prefix Prefix to look for
     * @return True if str starts with prefix
     */
    static bool starts_with(const std::string& str, const std::string& prefix);

    /**
     * Check if string ends with suffix
     * @param str String to check
     * @param suffix Suffix to look for
     * @return True if str ends with suffix
     */
    static bool ends_with(const std::string& str, const std::string& suffix);

    /**
     * Replace all occurrences of substring
     * @param str String to modify
     * @param from Substring to replace
     * @param to Replacement string
     * @return Modified string
     */
    static std::string replace_all(std::string str, const std::string& from, const std::string& to);
};

/**
 * Math utilities
 */
class Math {
public:
    /**
     * Greatest common divisor
     */
    template<typename T>
    static T gcd(T a, T b) {
        while (b != 0) {
            T temp = b;
            b = a % b;
            a = temp;
        }
        return a;
    }

    /**
     * Least common multiple
     */
    template<typename T>
    static T lcm(T a, T b) {
        return (a / gcd(a, b)) * b;
    }

    /**
     * Modular exponentiation (base^exp mod m)
     */
    static long long mod_pow(long long base, long long exp, long long mod);
};

} // namespace aoc
