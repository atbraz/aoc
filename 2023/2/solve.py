from pathlib import Path
from sys import argv, exit


def p1(input_: str) -> int:
    game_sum = 0
    for line in input_.splitlines():
        index, info = line.split(":")
        game = int(index.split(" ")[1])
        print("\n" + index)

        max_red, max_green, max_blue = 0, 0, 0
        sets = info.strip().split(";")
        for set_ in sets:
            print("\t" + set_.strip())
            cubes = set_.strip().split(", ")
            for cube in cubes:
                number, color = cube.strip().split(" ")
                number = int(number)
                match color:
                    case "red":
                        max_red = max(max_red, number)
                    case "green":
                        max_green = max(max_green, number)
                    case "blue":
                        max_blue = max(max_blue, number)
        print(f"\t{max_red=}")
        print(f"\t{max_green=}")
        print(f"\t{max_blue=}")
        if not any([max_red > 12, max_green > 13, max_blue > 14]):
            game_sum += game
            print(f"\n\t{index} possible")
        else:
            print(f"\n\t{index} not possible")
    return game_sum


def p2(input_: str) -> int:
    power_sum = 0
    for line in input_.splitlines():
        index, info = line.split(":")
        game = int(index.split(" ")[1])
        print("\n" + index)

        max_red, max_green, max_blue = 0, 0, 0
        sets = info.strip().split(";")
        for set_ in sets:
            print("\t" + set_.strip())
            cubes = set_.strip().split(", ")
            for cube in cubes:
                number, color = cube.strip().split(" ")
                number = int(number)
                match color:
                    case "red":
                        max_red = max(max_red, number)
                    case "green":
                        max_green = max(max_green, number)
                    case "blue":
                        max_blue = max(max_blue, number)
        print(f"\t{max_red=}")
        print(f"\t{max_green=}")
        print(f"\t{max_blue=}")

        power = max_red * max_green * max_blue
        print(f"\t{power=}")
        power_sum += power
    return power_sum


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
