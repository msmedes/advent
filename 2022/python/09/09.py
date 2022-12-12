def main():
    with open('../inputs/day09.txt') as f:
        
        lines = [parse_line(line.strip()) for line in f]
    answer1 = part1(lines)
    print(answer1)


def parse_line(line):
    direction, amount = line.split(" ")
    return direction, int(amount)

def do_motions(lines):
    chain = [(0,0)] * 10
    visited = set()
    for move in lines:
        chain = do_motion(move, chain, visited)
    return len(visited)

def get_delta(direction):
    deltas = None
    if direction == 'U':
        deltas = (0,1)
    elif direction == 'D':
        deltas = (0, -1)
    elif direction == 'R':
        deltas = (1, 0)
    elif direction == 'L':
        deltas = (-1, 0)
    return deltas


def do_motion(move, chain, visited):
    delta = get_delta(move[0])
    for _ in range(move[1]):
        chain = do_step(delta, chain)
        visited.add(chain[-1])

    return chain

def do_step(delta, chain):
    chain[0] = calc_position(chain[0], delta)
    for idx in range(len(chain)-1):
        head_pos, tail_pos = chain[idx], chain[idx+1]
        if not are_touching(head_pos, tail_pos):
            chain[idx+1] = move_tail(head_pos, tail_pos)
    return chain

def move_tail(head_pos, tail_pos):
    if head_pos[0] == tail_pos[0]:
        delta_x = 0
    if head_pos[1] == tail_pos[1]:
       delta_y = 0
    if head_pos[0] < tail_pos[0]:
        delta_x = -1
    if head_pos[0] > tail_pos[0]:
        delta_x = 1
    if head_pos[1] > tail_pos[1]:
        delta_y = 1 
    if head_pos[1] < tail_pos[1]:
        delta_y = -1
    return calc_position(tail_pos, (delta_x, delta_y))
    
def are_touching(head_pos, tail_pos):
    return abs(head_pos[0] - tail_pos[0]) <= 1 and abs(head_pos[1] - tail_pos[1]) <= 1

def calc_position(position, delta):
    return (position[0] + delta[0], position[1] + delta[1])

def part1(lines):
    unique_coords = do_motions(lines)
    return unique_coords

main()