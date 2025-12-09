# Advent of Code 2024 - Makefile
# Usage:
#   make day1          - Compile day 1
#   make run DAY=1     - Compile and run day 1 with input file
#   make test DAY=1    - Compile and run day 1 with sample input
#   make clean         - Remove all compiled binaries
#   make clean DAY=1   - Remove compiled binary for day 1

CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -O2
DAYS = 1 2 3 4

# Colors for output
GREEN = \033[0;32m
YELLOW = \033[0;33m
NC = \033[0m # No Color

# Default target
.PHONY: all
all: $(DAYS)

# Generic rule to compile any day
.PHONY: $(DAYS)
$(DAYS):
	@echo "$(GREEN)Compiling day $@...$(NC)"
	@$(CXX) $(CXXFLAGS) $@/main.cpp -o $@/day_$@

# Compile and run with input file
.PHONY: run
run:
ifndef DAY
	@echo "$(YELLOW)Usage: make run DAY=<day_number>$(NC)"
	@exit 1
endif
	@$(MAKE) $(DAY) --no-print-directory
	@echo "$(GREEN)Running day $(DAY) with input...$(NC)"
	@echo "$(DAY)/input" | ./$(DAY)/day_$(DAY)

# Compile and run with sample file
.PHONY: test
test:
ifndef DAY
	@echo "$(YELLOW)Usage: make test DAY=<day_number>$(NC)"
	@exit 1
endif
	@$(MAKE) $(DAY) --no-print-directory
	@echo "$(GREEN)Running day $(DAY) with sample input...$(NC)"
	@echo "$(DAY)/sample_1" | ./$(DAY)/day_$(DAY)

# Clean all or specific day
.PHONY: clean
clean:
ifdef DAY
	@echo "$(YELLOW)Cleaning day $(DAY)...$(NC)"
	@rm -f $(DAY)/day_$(DAY)
else
	@echo "$(YELLOW)Cleaning all compiled binaries...$(NC)"
	@rm -f */day_*
endif

# Help target
.PHONY: help
help:
	@echo "Advent of Code 2024 - Makefile Commands"
	@echo ""
	@echo "  make <day>         - Compile a specific day (e.g., make 1)"
	@echo "  make all           - Compile all days"
	@echo "  make run DAY=<n>   - Compile and run day <n> with input file"
	@echo "  make test DAY=<n>  - Compile and run day <n> with sample file"
	@echo "  make clean         - Remove all compiled binaries"
	@echo "  make clean DAY=<n> - Remove compiled binary for day <n>"
	@echo "  make help          - Show this help message"
