from src import part1, part2
from pathlib import Path
from sys import argv, exit

if len(argv) != 3:
    print("Usage: day1.py part path_to_input_file")
    exit()

INPUT_FILE = Path(argv[2]).resolve()
with INPUT_FILE.open() as file:
    INPUT = file.read()

if __name__ == "__main__":
    match argv[1]:
        case "part1":
            part1(INPUT)
        case "part2":
            part2(INPUT)
        case _:
            print("Invalid 'part' argument, try 'part1' or 'part2'")
