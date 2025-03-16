from pathlib import Path
from sys import argv, exit


def p1(input_: str) -> int:
    return 0


def p2(input_: str) -> int:
    return 0


def main():
    if len(argv) != 3:
        print("Usage: solve.py [ p1 | p2 ] path_to_input_file")
        exit()

    INPUT_FILE = Path(argv[2]).resolve()
    with INPUT_FILE.open() as file:
        INPUT = file.read()

    match argv[1]:
        case "p1":
            print(f"\nsolution: {p1(INPUT)}")
        case "p2":
            print(f"\nsolution: {p2(INPUT)}")
        case _:
            print("Invalid 'part' argument, try 'p1' or 'p2'")


if __name__ == "__main__":
    main()
