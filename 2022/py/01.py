def main():
    with open('../inputs/day01.txt') as f:
        elves = [[int(l) for l in line.split('\n')] for line in f.read().split("\n\n")]

    answer1 = part_1(elves)
    print('answer 1', answer1)
    answer2 = part_2(elves)
    print('anaswer 2', answer2)

def part_1(lines):
    return max(sum(elf) for elf in lines)

def part_2(lines):
    sums = [sum(elf) for elf in lines]
    return sum(sorted(sums, reverse=True)[:3])


if __name__ == '__main__':
    main()