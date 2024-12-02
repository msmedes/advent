def main():
    with open('../inputs/day03.txt') as f:
        compartments = [line.strip() for line in f]

    # answer1 = part1(rucksacks)
    # print(answer1)
    rucksacks = [set(line) for line in compartments]
    answer2 = part2(rucksacks)
    print(answer2)

def part1(rucksacks):
    priorities = [set1.intersection(set2).pop() for set1, set2 in rucksacks]
    return sum(get_value(priority) for priority in priorities)

def part2(rucksacks):
    priorities = []
    for i in range(0, len(rucksacks), 3):
        set1, set2, set3 = rucksacks[i], rucksacks[i+1], rucksacks[i+2]
        priorities.append(set1.intersection(set2, set3).pop())
    
    return sum(get_value(priority) for priority in priorities)

def get_value(char):
    if char.islower():
        return ord(char) - ord('a') + 1
    if char.isupper():
        return ord(char) - ord('A') + 27

main()