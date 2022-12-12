from collections import defaultdict
def main():
    with open('../inputs/day05.txt') as f:
        sections = f.read().split("\n\n")
    
    input_stacks = sections[0]
    input_moves = sections[1].split('\n')

    stacks = build_stacks(input_stacks)
    moves = get_moves(input_moves)
    answer1 = part1(stacks, moves)
    print('answer1', answer1)

    input_stacks = sections[0]
    input_moves = sections[1].split('\n')
    stacks = build_stacks(input_stacks)
    moves = get_moves(input_moves)
    answer2 = part2(stacks, moves)
    print('answer2', answer2)



def build_stacks(raw_input):
    lines = raw_input.split('\n')[:-1]
    stacks = defaultdict(list)
    for y in range(len(lines)):
        row = lines[y][1::4]
        for idx, char in enumerate(row):
            if char != " ":
                stacks[idx].insert(0, char)
    return stacks

def get_moves(raw_input):
    return [[int(v) for v in line.replace('move ', ' ').replace(' from ', ' ').replace(' to ', ' ').split(' ')[1:]] for line in raw_input]


def do_moves(stacks, moves):
    for move in moves:
        amount, _from, to = move[0], move[1]-1, move[2]-1
        for _ in range(amount):
            stacks[to].append(stacks[_from].pop())

    return stacks

def do_moves_2(stacks, moves):
    for move in moves:
        amount, _from, to = move[0], move[1]-1, move[2]-1
        to_stack_len = len(stacks[to])
        for _ in range(amount):
            stacks[to].insert(to_stack_len, stacks[_from].pop())

    return stacks

def part1(stacks, moves):
    stacks = do_moves(stacks, moves)
    return ''.join(stacks[stack][-1] for stack in sorted(stacks))

def part2(stacks, moves):
    stacks = do_moves_2(stacks, moves)
    return ''.join(stacks[stack][-1] for stack in sorted(stacks))

main()