def main():
    with open('../inputs/day06.txt') as f:
        buffer = [c for c in f.read()]


    answer1 = part1(buffer)
    print('answer1', answer1)


def part1(buffer):
    start, end = 0, 14
    uniques = set(buffer[start:end])

    while len(uniques) != 14:
        uniques.remove(buffer[start])
        start += 1
        end += 1
        uniques = set(buffer[start:end])

    return end

main()