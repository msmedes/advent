import re

def process_input():
    with open("input.txt", "r") as f:
        return f.read()

def get_multipliers(mul):
    mul = mul.replace('mul(', '')
    mul = mul.replace(')', '')
    return [int(x) for x in mul.split(',')]

def multiply_multipliers(mul):
    multipliers = get_multipliers(mul)
    return multipliers[0] * multipliers[1]

def part_1(instructions):
    muls = match_mul(instructions)
    return sum(multiply_multipliers(mul) for mul in muls)

def part_2(instructions):
    matches = match_mul_with_commands(instructions)
    total = 0
    we_are = True
    for match in matches:
        if match == "don't()":
            we_are = False
            continue
        elif match == "do()":
            we_are = True
            continue
        if we_are:
            total += multiply_multipliers(match)
    return total



def match_mul(text):
    return re.findall(r'mul\(\d{1,3},\d{1,3}\)', text)

def match_mul_with_commands(text):
    return re.findall(r"(mul\(\d{1,3},\d{1,3}\)|(?:do|don't)\(\))", text)

def main():
    data = process_input()
    part1 = part_1(data)
    part2 = part_2(data)
    print(f"Part 1: {part1}")
    print(f"Part 2: {part2}")

if __name__ == "__main__":
    main()