#include "aoc_common.hpp"
#include <cctype>

namespace aoc {

// InputReader implementations

std::string InputReader::as_string(const std::string& filename) {
    std::ifstream file(filename);
    if (!file) {
        throw std::runtime_error("Cannot open file: " + filename);
    }

    std::stringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
}

std::vector<std::string> InputReader::as_lines(const std::string& filename) {
    std::ifstream file(filename);
    if (!file) {
        throw std::runtime_error("Cannot open file: " + filename);
    }

    std::vector<std::string> lines;
    std::string line;
    while (std::getline(file, line)) {
        lines.push_back(line);
    }

    return lines;
}

std::vector<std::vector<char>> InputReader::as_char_grid(const std::string& filename) {
    auto lines = as_lines(filename);
    std::vector<std::vector<char>> grid;

    for (const auto& line : lines) {
        std::vector<char> row(line.begin(), line.end());
        grid.push_back(row);
    }

    return grid;
}

std::vector<std::string> InputReader::as_paragraphs(const std::string& filename) {
    std::string content = as_string(filename);
    std::vector<std::string> paragraphs;
    std::stringstream ss(content);
    std::string paragraph;
    std::string line;

    while (std::getline(ss, line)) {
        if (line.empty() || std::all_of(line.begin(), line.end(), ::isspace)) {
            if (!paragraph.empty()) {
                paragraphs.push_back(paragraph);
                paragraph.clear();
            }
        } else {
            if (!paragraph.empty()) {
                paragraph += '\n';
            }
            paragraph += line;
        }
    }

    if (!paragraph.empty()) {
        paragraphs.push_back(paragraph);
    }

    return paragraphs;
}

std::string InputReader::as_single_line(const std::string& filename) {
    auto lines = as_lines(filename);
    std::string result;
    for (const auto& line : lines) {
        result += line;
    }
    return result;
}

// StringUtils implementations

std::vector<std::string> StringUtils::split(const std::string& str, char delimiter) {
    std::vector<std::string> tokens;
    std::stringstream ss(str);
    std::string token;

    while (std::getline(ss, token, delimiter)) {
        tokens.push_back(token);
    }

    return tokens;
}

std::string StringUtils::trim(const std::string& str) {
    return ltrim(rtrim(str));
}

std::string StringUtils::ltrim(const std::string& str) {
    auto it = std::find_if(str.begin(), str.end(), [](unsigned char ch) {
        return !std::isspace(ch);
    });
    return std::string(it, str.end());
}

std::string StringUtils::rtrim(const std::string& str) {
    auto it = std::find_if(str.rbegin(), str.rend(), [](unsigned char ch) {
        return !std::isspace(ch);
    });
    return std::string(str.begin(), it.base());
}

bool StringUtils::starts_with(const std::string& str, const std::string& prefix) {
    if (prefix.length() > str.length()) {
        return false;
    }
    return str.compare(0, prefix.length(), prefix) == 0;
}

bool StringUtils::ends_with(const std::string& str, const std::string& suffix) {
    if (suffix.length() > str.length()) {
        return false;
    }
    return str.compare(str.length() - suffix.length(), suffix.length(), suffix) == 0;
}

std::string StringUtils::replace_all(std::string str, const std::string& from, const std::string& to) {
    size_t pos = 0;
    while ((pos = str.find(from, pos)) != std::string::npos) {
        str.replace(pos, from.length(), to);
        pos += to.length();
    }
    return str;
}

// Math implementations

long long Math::mod_pow(long long base, long long exp, long long mod) {
    long long result = 1;
    base %= mod;

    while (exp > 0) {
        if (exp & 1) {
            result = (result * base) % mod;
        }
        base = (base * base) % mod;
        exp >>= 1;
    }

    return result;
}

} // namespace aoc
