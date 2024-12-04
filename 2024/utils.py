from typing import Generator


def all_coords(coord: tuple[int, int]) -> Generator[tuple[int, int], None, None]:
    for coord in diagonal_coords(coord):
        yield coord
    for coord in manhattan_coords(coord):
        yield coord


def diagonal_coords(coord: tuple[int, int]) -> Generator[tuple[int, int], None, None]:
    for x, y in [(-1, -1), (1, -1), (1, 1), (-1, 1)]:
        yield x, y


def manhattan_coords(coord: tuple[int, int]) -> Generator[tuple[int, int], None, None]:
    for x, y in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        yield x, y
