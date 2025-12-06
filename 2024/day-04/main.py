import os
import sys

# python is super cool thanks python
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import utils


def main():
    data = process_input()
    width = len(data[0])
    height = len(data)
    word_search = {(x, y): data[y][x] for y in range(height) for x in range(width)}
    part1 = part_1(word_search)
    part2 = part_2(word_search)
    print(f"Part 1: {part1}")
    print(f"Part 2: {part2}")


def process_input():
    with open("input.txt", "r") as f:
        return [list(line.strip()) for line in f.readlines()]


cycle = {"X": "M", "M": "A", "A": "S"}


def traverse_from_coord(
    curr_coord: tuple[int, int], x_delta: int, y_delta: int, word_search
):
    next_coord = (curr_coord[0] + x_delta, curr_coord[1] + y_delta)
    curr_char = word_search[curr_coord]
    if curr_char == "S":
        return 1
    if next_coord not in word_search:
        return 0
    next_char = word_search.get(next_coord, None)
    if next_char == cycle.get(curr_char, None):
        return traverse_from_coord(next_coord, x_delta, y_delta, word_search)
    else:
        return 0


def part_1(word_search):
    x_coords = [key for key, value in word_search.items() if value == "X"]
    return sum(
        traverse_from_coord(coord, x_delta, y_delta, word_search)
        for coord in x_coords
        for x_delta, y_delta in utils.all_deltas(coord)
    )


sequence = ["M", "M", "S", "S"]
rotations = {tuple(sequence[i:] + sequence[:i]) for i in range(len(sequence))}


def check_pattern(coord, word_search):
    diagonal_coords = [
        (coord[0] + x, coord[1] + y) for x, y in utils.diagonal_deltas(coord)
    ]
    diagonal_sequence = {
        tuple(
            word_search.get(diagonal_coord, None) for diagonal_coord in diagonal_coords
        )
    }
    return len(diagonal_sequence & rotations) == 1


def part_2(word_search):
    a_coords = [key for key, value in word_search.items() if value == "A"]
    return sum(1 for coord in a_coords if check_pattern(coord, word_search))


if __name__ == "__main__":
    main()
