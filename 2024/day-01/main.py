from collections import Counter


def process_input() -> tuple[list[int], list[int]]:
    with open("input.txt", "r") as file:
        lines = file.readlines()
        left_list, right_list = [], []
        for line in lines:
            left, right = line.split("   ")
            left_list.append(int(left))
            right_list.append(int(right))
    return left_list, right_list


def main():
    input_lists = process_input()
    # ans = part1(input_lists)
    # print(ans)
    ans = part2(input_lists)
    print(ans)


def part1(lists: tuple[list[int], list[int]]) -> int:
    left_list, right_list = lists
    return sum(
        abs(left - right)
        for left, right in zip(
            sorted(left_list, reverse=True), sorted(right_list, reverse=True)
        )
    )


def part2(lists: tuple[list[int], list[int]]):
    left_counts = Counter(lists[0])
    right_counts = Counter(lists[1])

    return sum(left * right_counts.get(left, 0) for left in left_counts)


if __name__ == "__main__":
    main()
