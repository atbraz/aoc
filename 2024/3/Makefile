# Makefile for Day 3
# Usage:
#   make        - Compile the solution
#   make run    - Compile and run with input file
#   make test   - Compile and run with sample input
#   make clean  - Remove compiled binary

CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -O2
DAY = 3
BINARY = day_$(DAY)

# Colors for output
GREEN = \033[0;32m
YELLOW = \033[0;33m
NC = \033[0m # No Color

.PHONY: all run test clean

# Default target - compile
all: $(BINARY)

$(BINARY): main.cpp
	@echo "$(GREEN)Compiling day $(DAY)...$(NC)"
	@$(CXX) $(CXXFLAGS) main.cpp -o $(BINARY)

# Run with input file
run: $(BINARY)
	@echo "$(GREEN)Running day $(DAY) with input...$(NC)"
	@echo "input" | ./$(BINARY)

# Run with sample file
test: $(BINARY)
	@echo "$(GREEN)Running day $(DAY) with sample input...$(NC)"
	@echo "sample_1" | ./$(BINARY)

# Clean compiled binary
clean:
	@echo "$(YELLOW)Cleaning day $(DAY)...$(NC)"
	@rm -f $(BINARY)
