from typing import Generator


def all_deltas(coord: tuple[int, int]) -> Generator[tuple[int, int], None, None]:
    for coord in diagonal_deltas(coord):
        yield coord
    for coord in manhattan_deltas(coord):
        yield coord


def diagonal_deltas(coord: tuple[int, int]) -> Generator[tuple[int, int], None, None]:
    for x, y in [(-1, -1), (1, -1), (1, 1), (-1, 1)]:
        yield x, y


def manhattan_deltas(coord: tuple[int, int]) -> Generator[tuple[int, int], None, None]:
    for x, y in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        yield x, y
