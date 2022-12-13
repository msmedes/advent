from heapq import heappop, heappush
def main():
    with open('inputs/day12.txt') as f:
        grid = [[c for c in line.strip()] for line in f]

    answer1 = part1(grid)
    print(answer1)
    answer2 = part2(grid)
    print(answer2)

def part1(grid):
    start, end = get_start_and_end(grid, 'S', 'E')
    print('start, end', start, end)
    grid[end[1]][end[0]] = 'z'
    grid[start[1]][start[0]] = 'a'
    path = bfs(start, end, grid)
    return path

def part2(grid):
    start, end = get_start_and_end(grid, 'S', 'E')
    grid[end[1]][end[0]] = 'z'
    grid[start[1]][start[0]] = 'a' 
    all_as_baby = get_as(grid)
    return min(bfs(a, end, grid) for a in all_as_baby if a)

def get_as(grid):
    a_s = []
    for y in range(len(grid)):
        for x in range(len(grid[0])):
            curr_value = grid[y][x]
            if curr_value == 'a':
                a_s.append((x,y))
    return a_s


def valid_coord(x, y, grid):
    return 0 <= y < len(grid) and 0 <= x < len(grid[0])


def bfs(start, end, grid):
    queue = [(0, start)]
    visited = set()
    while queue:
        distance, curr_coord = heappop(queue)
        if curr_coord in visited:
            continue
        if curr_coord == end:
            return distance
        visited.add(curr_coord)
        current_value = grid[curr_coord[1]][curr_coord[0]]
        for deltas in [(1,0), (-1, 0), (0, 1), (0,-1)]:
            x, y = (curr_coord[0] + deltas[0], curr_coord[1] + deltas[1])
            if valid_coord(x, y, grid) and\
                ord(grid[y][x]) - ord(current_value) < 2:
                heappush(queue, (distance+1, (x,y)))
    return float('inf')




def get_start_and_end(grid, start_char, end_char):
    start = end = None
    for y in range(len(grid)):
        for x in range(len(grid[0])):
            if grid[y][x] == start_char:
                start = (x, y)
            if grid[y][x] == end_char:
                end = (x, y)
    return start, end


main()