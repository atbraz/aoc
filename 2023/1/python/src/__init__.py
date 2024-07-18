digit_names: dict[str, int] = {
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


def part1(input_: str):
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
                leftmost = digit if leftmost < 0 else leftmost
                rightmost = digit
            else:
                digits.append("-")

        print("\t" + "".join(digits))
        print(f"\t{leftmost=}")
        print(f"\t{rightmost=}")

        value = (leftmost * 10) + rightmost
        print(f"\t{value=}")

        total_sum += value

    print(total_sum)


def part2(input_: str):
    total_sum = 0
    for i, line in enumerate(input_.splitlines()):
        print(f"Line {i}: \n\t{line}")

        leftmost: int = -1
        rightmost: int = -1
        value: int = 0
        digits: list[str] = []
        running_str: str = ""

        for char in list(line):
            running_str += char
            matches = [name in running_str for name in digit_names.keys()]
            print(matches)
            if char.isdigit():
                digits.append("*")
                digit = int(char)
                leftmost = digit if leftmost < 0 else leftmost
                rightmost = digit
            else:
                digits.append("-")

        print("\t" + "".join(digits))
        print(f"\t{leftmost=}")
        print(f"\t{rightmost=}")

        value = (leftmost * 10) + rightmost
        print(f"\t{value=}")

        total_sum += value

    print(total_sum)
