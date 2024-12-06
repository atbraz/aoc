CXX = clang++-17
CXXFLAGS = -std=c++17 -Wall -Wextra

UTILS_DIR = ../cpp_utils
SOURCES = main.cpp $(UTILS_DIR)/parse_input.cpp
TARGET = bin/solution

$(TARGET): $(SOURCES)
	@mkdir -p bin
	$(CXX) $(CXXFLAGS) $(SOURCES) -o $(TARGET)

.PHONY: run
run: $(TARGET)
	./$(TARGET)

.PHONY: clean
clean:
	rm -f $(TARGET)
