from collections import defaultdict

def main():
    with open('../inputs/day10.txt') as f:
        lines = [line.strip().split(" ") for line in f]
        lines = [el for line in lines for el in line]

    answer1 = part1(lines)
    print(answer1)
    part2(lines)

def part1(lines):
    signal_strengths = get_signal_strengths(lines)
    return sum([signal_strengths[cycle] for cycle in range(20, len(signal_strengths),40)])


def get_signal_strengths(lines):
    register = 1
    cycle = 1
    signal_strengths = defaultdict(int)
    for line in lines:
        cycle += 1
        if line not in {'addx', 'noop'}:
            register += int(line)
        signal_strengths[cycle] = cycle * register
    return signal_strengths

def part2(lines):
    draw_signal(lines)

def draw_signal(lines):
    register = 1
    cycle = 0
    for line in lines:
        cycle += 1
        if line not in {'addx', 'noop'}:
            register += int(line)
        mod = cycle  % 40
        if mod >= register-1 and mod <= register+1:
            print("#", end="")
        else:
            print('.', end="")
        if cycle % 40 == 0:
            print('')
 

main()