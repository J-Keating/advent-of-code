from functools import reduce
from operator import mul
from typing import List

DAY = 'd3'

def load_data(file_path) -> List[List[str]]:
    with open(file_path, 'r') as file:
        return [list(line.strip()) for line in file]

def part1():
    data = load_data(DAY + '\\data_real.txt')
    width = len(data[0])
    count = len([row for row in range(len(data)) if data[row][(3*row)%width] == '#'])
    print(f"part1: {count}")

def count_given_steps(data, step_down_right):
    (height, width) = (len(data), len(data[0]))
    count = 0
    pos = (0, 0)
    while pos[0] < height:
        if data[pos[0]][pos[1]] == '#':
            count += 1
        pos = (pos[0] + step_down_right[0], (pos[1] + step_down_right[1]) % width)
    return count

def part2():
    data = load_data(DAY + '\\data_real.txt')
    slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
    tree_counts = list(map(lambda slope: count_given_steps(data, slope), slopes))
    total = reduce(mul, tree_counts, 1)
    print(f"part1: {total}")
    

part1()
part2()

# part1: 195
# part1: 3772314000