from collections import Counter
def main():
    with open('../inputs/day04.txt') as f:
        lines = [line.strip() for line in f]
        pairs = [line.split(',') for line in lines]
        pairs = [[line.split('-') for line in lines] for lines in pairs]
        pairs = [[[int(x) for x in i] for i in lines] for lines in pairs]

    answer1 = part1(pairs)
    print(answer1)
    # answer2 = part2(pairs)
    # print(answer2)


def part1(lines):
    return len([line for line in lines if contained(line)])

def part2(lines):
    return len([line for line in lines if overlapping(line)])

def contained(line):
    pair1, pair2 = line[0], line[1]
    return (pair1[0] <= pair2[0] and pair1[1] >= pair2[1]) or (pair2[0] <= pair1[0] and pair2[1]>= pair1[1])

def overlapping(line):
    pair1, pair2 = line[0], line[1]
    all_indexes = [i for i in range(pair1[0], pair1[1]+1)] + [i for i in range(pair2[0], pair2[1]+1)]
    counts = Counter(all_indexes)
    return any(val for val in counts.values() if val > 1) 

main()