def process_input():
    with open("input.txt", "r") as f:
        return [[int(x) for x in line.strip().split()] for line in f.readlines()]


def increasing_safely(report: list[int]):
    for l, r in zip(report, report[1:]):
        diff = r - l
        if not 1 <= diff <= 3:
            return False
    return True


def decreasing_safely(report: list[int]):
    for l, r in zip(report, report[1:]):
        diff = l - r
        if not 1 <= diff <= 3:
            return False
    return True


def is_report_safe(report: list[int]):
    return increasing_safely(report) or decreasing_safely(report)


def part_1(data: list[list[int]]):
    return sum(1 for report in data if is_report_safe(report))


def part_2(data: list[list[int]]):
    return sum(
        1
        for report in data
        if is_report_safe(report)
        or any(is_report_safe(report[:i] + report[i + 1 :]) for i in range(len(report)))
    )


def main():
    data = process_input()
    part_1_ans = part_1(data)
    part_2_ans = part_2(data)
    print(f"Part 1: {part_1_ans}")
    print(f"Part 2: {part_2_ans}")


if __name__ == "__main__":
    main()
