from pathlib import Path
from sys import argv, exit


DIGIT_NAMES: dict[str, int] = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}


def part_1(input_: str):
    total_sum = 0
    for i, line in enumerate(input_.splitlines()):
        print(f"Line {i}: \n\t{line}")

        leftmost: int = -1
        rightmost: int = -1
        value: int = 0
        digits: list[str] = []

        for char in list(line):
            if char.isdigit():
                digits.append("*")
                digit = int(char)
                leftmost, rightmost = _update_values(digit, leftmost, rightmost)
            else:
                digits.append("-")

        print("\t" + "".join(digits))
        print(f"\t{leftmost=}")
        print(f"\t{rightmost=}")

        value = (leftmost * 10) + rightmost
        print(f"\t{value=}")

        total_sum += value

    print(f"\n{total_sum=}")


def part_2(input_: str):
    total_sum = 0
    for i, line in enumerate(input_.splitlines()):
        print(f"Line {i}: \n\t{line}")

        leftmost: int = -1
        rightmost: int = -1
        line_value: int = 0

        digit_line: list[str] = []
        running_str: str = ""
        line_len = len(line)
        for i, char in enumerate(list(line)):
            if char.isdigit():
                digit_line.append(char)
                digit = int(char)
                leftmost, rightmost = _update_values(digit, leftmost, rightmost)
                continue
            else:
                digit_line.append("-")

            running_str += char
            for name in DIGIT_NAMES:
                if name in "".join(running_str[-len(name) :]):
                    digit = DIGIT_NAMES[name]
                    leftmost, rightmost = _update_values(digit, leftmost, rightmost)
                    print(
                        "\t"
                        + "-" * ((i + 1) - len(name))
                        + name
                        + "-" * (line_len - (i + 1))
                    )

        print("\t" + "".join(digit_line))

        print(f"\n\t{leftmost=}")
        print(f"\t{rightmost=}")

        line_value = (leftmost * 10) + rightmost
        print(f"\t{line_value=}\n")

        total_sum += line_value

    print(f"{total_sum=}")


def _update_values(digit: int, leftmost: int, rightmost: int) -> tuple[int, int]:
    return digit if leftmost < 0 else leftmost, digit


def main():
    if len(argv) != 3:
        print("Usage: day_1.py [ part_1 | part_2 ] path_to_input_file")
        exit()

    INPUT_FILE = Path(argv[2]).resolve()
    with INPUT_FILE.open() as file:
        INPUT = file.read()

    match argv[1]:
        case "part_1":
            part_1(INPUT)
        case "part_2":
            part_2(INPUT)
        case _:
            print("Invalid 'part' argument, try 'part_1' or 'part_2'")


if __name__ == "__main__":
    main()
