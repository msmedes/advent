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
    should_multiply = True
    
    # imperative and I HATE IT
    for operation in matches:
        match operation:
            case "don't()":
                should_multiply = False
            case "do()":
                should_multiply = True
            case mul if should_multiply:
                total += multiply_multipliers(mul)
                
    return total

def part_2_recursive_helper(instructions, should_multiply, total):
    if not instructions:
        return total
    match instructions[0]:
        case "don't()":
            should_multiply = False
        case "do()":
            should_multiply = True
        case mul:
            if should_multiply:
                total += multiply_multipliers(mul)
    return part_2_recursive_helper(instructions[1:], should_multiply, total)

def part_2_recursive(instructions):
    matches = match_mul_with_commands(instructions)
    return part_2_recursive_helper(matches, True, 0)

def match_mul(text):
    return re.findall(r'mul\(\d{1,3},\d{1,3}\)', text)

def match_mul_with_commands(text):
    return re.findall(r"(mul\(\d{1,3},\d{1,3}\)|(?:do|don't)\(\))", text)

def main():

    data = process_input()
    part1 = part_1(data)
    part2 = part_2_recursive(data)
    print(f"Part 1: {part1}")
    print(f"Part 2: {part2}")

if __name__ == "__main__":
    main()