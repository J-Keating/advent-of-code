import inspect
import time

DAY = 'd12'

class TEST_DATA:
    FILE_NAME = 'data_test.txt'

class REAL_DATA:
    FILE_NAME = 'data_real.txt'

class Directions:
    N = 0
    E = 1
    S = 2
    W = 3
    COUNT = 4
    to_vec = {
        N: (0, -1),
        S: (0, 1),
        E: (1, 0),
        W: (-1, 0)
    }
    to_dir = {
        'N': N,
        'E': E,
        'S': S,
        'W': W
    }

def load_data(file_path):
    with open(file_path, 'r') as file:
        return [line.strip() for line in file]

def part1():
    start_time = time.perf_counter()
    data = REAL_DATA
    instructions = load_data(DAY + '\\' + data.FILE_NAME)
    loc = (0, 0)
    face_direction = Directions.E
    for instruction in instructions:
        action = instruction[0]
        value = int(instruction[1:])
        move_direction = (0, 0)
        match action:
            case 'N' | 'S' | 'E' | 'W':
                move_direction = Directions.to_vec[Directions.to_dir[action]]
            case 'L':
                face_direction = (face_direction - value // 90) % Directions.COUNT
            case 'R':
                face_direction = (face_direction + value // 90) % Directions.COUNT
            case 'F':
                move_direction = Directions.to_vec[face_direction]
        loc = (loc[0] + move_direction[0] * value, loc[1] + move_direction[1] * value)
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