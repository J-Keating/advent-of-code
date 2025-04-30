import inspect
import time

DAY = 'd12'

class TEST_DATA:
    FILE_NAME = 'data_test.txt'

class REAL_DATA:
    FILE_NAME = 'data_real.txt'

def load_data(file_path):
    with open(file_path, 'r') as file:
        return [list(line.strip()) for line in file]

def part1():
    start_time = time.perf_counter()
    data = REAL_DATA
    instructions = load_data(DAY + '\\' + data.FILE_NAME)
    loc = (0, 0)
    direction = 0
    for instruction in instructions:
        action = instruction[0]
        value = int(instruction[1:])
        match action:
            case 'N':
                loc = (loc[0] - value, loc[1])
            case 'S':
                loc = (loc[0] + value, loc[1])
            case 'E':
                loc = (loc[0], loc[1] + value)
            case 'W':
                loc = (loc[0], loc[1] - value)
            case 'L':
                direction = (direction - value // 90) % 4
            case 'R':
                direction = (direction + value // 90) % 4
            case 'F':
                if direction == 0:
                    loc = (loc[0] - value, loc[1])
                elif direction == 1:
                    loc = (loc[0], loc[1] + value)
                elif direction == 2:
                    loc = (loc[0] + value, loc[1])
                elif direction == 3:
                    loc = (loc[0], loc[1] - value)
    manhattan_dist = abs(loc[0]) + abs(loc[1])                   
    elapsed_time = time.perf_counter() - start_time
    print(f"{inspect.currentframe().f_code.co_name} ({elapsed_time:.2f} sec) : {manhattan_dist}")

def part2():
    start_time = time.perf_counter()
#    data = REAL_DATA
    elapsed_time = time.perf_counter() - start_time
    print(f"{inspect.currentframe().f_code.co_name} ({elapsed_time:.2f} sec) : {0}")

part1()
part2()

# part1: 2261
# part2: 2039