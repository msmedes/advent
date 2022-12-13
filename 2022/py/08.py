def main():
    with open('../inputs/day08.txt') as f:
        grid = [[int(c) for c in line.strip()] for line in f]

    answer1 = part1(grid)
    print('answer_1', answer1)
    answer2 = part2(grid)
    print('answer_2', answer2)


def part1(grid):
    count = 0
    width, height = len(grid[0]), len(grid)
    edge_count = 2 * width + 2 * height - 4
    for y in range(1, height-1):
        for x in range(1, width-1):
            if is_visible(x, y, grid):
                count += 1

    return count + edge_count

def part2(grid):
    scenic_scores = []
    width, height = len(grid[0]), len(grid)
    for y in range(height):
        for x in range(width):
            scenic_scores.append(scenic_score(x, y, grid))
    return max(scenic_scores)

def scenic_score(x, y, grid):
    scenic_score = 1
    tree_height = grid[y][x]
    for x_delta, y_delta in delta_iterator():
        scenic_score *= get_viewing_distance(tree_height, x, y, x_delta, y_delta, grid)

    return scenic_score


def get_viewing_distance(tree_height, x, y, x_delta, y_delta, grid):
    i = directional_iterator(x, y, x_delta, y_delta, grid)
    distance = 0
    while i:
        try:
            other_tree = next(i)
            distance += 1
            if other_tree >= tree_height:
                break
        except StopIteration:
            break
    return distance

    
def is_visible(x, y, grid):
    tree_height = grid[y][x]
    for x_delta, y_delta in delta_iterator():
        if can_see_from_side(tree_height, x, y, x_delta, y_delta, grid):
            return True
    
    return False


def delta_iterator():
    for deltas in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        yield deltas


def directional_iterator(x, y, x_delta, y_delta, grid):
    width, height = len(grid[0]), len(grid)
    x += x_delta
    y += y_delta
    while x < width and x >=0 and y >=0 and y < height:
        value = grid[y][x]
        x += x_delta
        y += y_delta
        yield value

def can_see_from_side(tree_height, x, y, x_delta, y_delta, grid):
    i = directional_iterator(x, y, x_delta, y_delta, grid)
    while i:
        try:
            other_tree = next(i)
            if other_tree >= tree_height:
                return False
        except StopIteration:
            return True

main()