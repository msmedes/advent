same = {
    'A': 'X',
    'B': 'Y',
    'C': 'Z'
}

values = {
    'X': 1,
    'Y': 2,
    'Z': 3
}

win = {
    'A': 'Y',
    'B': 'Z',
    'C': 'X'
}

lose = {
    'A': 'Z',
    'B': 'X',
    'C': 'Y'
}

def main():
    with open('../inputs/day02.txt') as f:
        lines = [line.strip() for line in f]
        rounds = [line.split(" ") for line in lines]
        
    answer_1 = part1(rounds)
    print(answer_1)
    answer_2 = part2(rounds)
    print(answer_2)

def part1(rounds):
    score = 0
    for them, me in rounds:
        if me == win[them]:
            score += values[me] + 6
        elif me == same[them]:
            score += values[me] + 3
        elif me != win[them]:
            score += values[me] + 0
    return score

def part2(rounds):
    modded = []
    for them, me in rounds:
        if me == 'Y':
            modded.append([them, same[them]])
        if me == 'X':
            modded.append([them, lose[them]])
        if me == 'Z':
            modded.append([them, win[them]])

    return part1(modded)



if __name__ == '__main__':
    main()