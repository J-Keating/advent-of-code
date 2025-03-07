# Load data from real_data.txt

import itertools

DAY = 'd1'

def load_data(file_path):
    with open(file_path, 'r') as file:
        numbers = [int(line.strip()) for line in file]
    return numbers

def part1():
    data = load_data(DAY + '\\data_real.txt')
    for n1, n2 in itertools.combinations(data, 2):
        if n1 + n2 == 2020:
            print(f"part1: {n1} and {n2} : '+' = {n1 + n2} : '*' = {n1 * n2}")
            return

def part2():
    data = load_data(DAY + '\\data_real.txt')
    for n1, n2, n3 in itertools.combinations(data, 3):
        if n1 + n2 + n3 == 2020:
            print(f"part2: {n1} and {n2} and {n3} : '+' = {n1 + n2 + n3} :  '*' = {n1 * n2 * n3}")
            return

part1()
part2()

# part1: 1162 and 858 : '+' = 2020 : '*' = 996996
# part2: 282 and 19 and 1719 : '+' = 2020 :  '*' = 9210402