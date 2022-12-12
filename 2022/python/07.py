from collections import defaultdict
def main():
    with open('../inputs/day07.txt') as f:
        lines = [line.strip() for line in f]
    
    answer1 = part1(lines)
    print('answer1', answer1)
    answer2 = part2(lines)
    print('answer2', answer2)


def part1(lines):
    dir_sizes =  build_directories(lines)
    return sum(size for size in dir_sizes.values() if size <= 100_000)
    
def part2(lines):
    total_available = 70000000
    needed = 30000000
    dirs = build_directories(lines)
    unused = total_available - dirs['/']
    return min(val for val in dirs.values() if val + unused >= needed)




def build_directories(lines):
    path = []
    dir_size = defaultdict(int)
    for line in lines:
        if line.startswith('$ ls'):
            continue
        elif line.startswith('dir'):
            continue
        elif line.startswith("$ cd"):
            dest = line[5:]
            if dest == '/':
                path = ['/']
            elif dest == '..':
                path.pop()
            else:
                print(''.join(p for p in path) + dest + '/')
                path.append(''.join(p for p in path) + '/' + dest)
        else:
            size, _filename = line.split()
            for directory in path:
                dir_size[directory] += int(size)
    
    return dir_size
main()
