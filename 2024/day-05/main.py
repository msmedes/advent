from collections import defaultdict


def process_input():
    with open("test_input.txt", "r") as f:
        rules, updates = f.read().split("\n\n")
        rules = [
            [int(x) for x in rule.strip().split("|")] for rule in rules.splitlines()
        ]
        updates = [
            [int(x) for x in update.strip().split(",")]
            for update in updates.splitlines()
        ]
        return rules, updates


# I probably need to do topological sort when creating the graph?


def is_after(curr_page, next_page, rule_map):
    print("here we go")
    visited = set()
    stack = [curr_page]
    while stack:
        print("stack", stack)
        curr_page = stack.pop()
        visited.add(curr_page)
        for page in rule_map.get(curr_page, []):
            if page == next_page:
                return True
            if page not in visited:
                stack.append(page)
    return False


def valid_update(updates, rule_map):
    if len(updates) == 1:
        return True
    if is_after(updates[0], updates[1], rule_map):
        return valid_update(updates[1:], rule_map)
    return False


def part_1(rules, updates):
    print("rules", rules)
    valid_updates = [update for update in updates if valid_update(update, rules)]
    middles = [update[len(update) // 2] for update in valid_updates]
    return sum(middles)


def part_2(rules, updates):
    pass


def main():
    rules, updates = process_input()
    rule_map = defaultdict(set)
    for rule in rules:
        rule_map[rule[0]].add(rule[1])
    result_1 = part_1(rule_map, updates)
    result_2 = part_2(rule_map, updates)
    print(f"Part 1: {result_1}")
    print(f"Part 2: {result_2}")


if __name__ == "__main__":
    main()
