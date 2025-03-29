import inspect

DAY = 'd11'
FULL_SEAT = '#'
EMPTY_SEAT = 'L'
FLOOR = '.'

class TEST_DATA:
    FILE_NAME = 'data_test.txt'

class REAL_DATA:
    FILE_NAME = 'data_real.txt'

def load_data(file_path):
    with open(file_path, 'r') as file:
        return [list(line.strip()) for line in file]
    
def print_array(array):
    for row in array:
        print(''.join(row))

def in_bounds(array, pos):
    return pos[0] >= 0 and pos[0] < len(array) and pos[1] >= 0 and pos[1] < len(array[0])

def count_adjacent_occupied(seats, pos):
    NEIGHBORS = [   (-1, -1), (0, -1), (1, -1),
                    (-1,  0),          (1,  0),
                    (-1,  1), (0,  1), (1,  1)]
    occupied = [n for n in NEIGHBORS if in_bounds(seats, (pos[0] + n[0],pos[1] + n[1])) and seats[pos[0] + n[0]][pos[1] + n[1]] == FULL_SEAT]
    return len(occupied)

def new_seat_state(seats, pos):
    match seats[pos[0]][pos[1]]:
        case x if x == FLOOR:
            return None
        case x if x == EMPTY_SEAT:
            return FULL_SEAT if count_adjacent_occupied(seats, pos) == 0 else None
        case x if x == FULL_SEAT:
            return EMPTY_SEAT if count_adjacent_occupied(seats, pos) >= 4 else None

def part1():
    data = REAL_DATA
    seats = load_data(DAY + '\\' + data.FILE_NAME)
    while True:
        changes = []
        for r in range(0, len(seats)):
            for c in range(0, len(seats[0])):
                loc = (r, c)
                new_state = new_seat_state(seats, loc)
                if new_state:
                    changes.append((loc, new_state))
        if len(changes) == 0:
            break
        for loc, new_state in changes:
            seats[loc[0]][loc[1]] = new_state
        #print_array(seats)
    occupied = sum(len([s for s in row if s == FULL_SEAT]) for row in seats)
    print(f"{inspect.currentframe().f_code.co_name}: {occupied}")

def part2():
    data = TEST_DATA
    seats = load_data(DAY + '\\' + data.FILE_NAME)
    print(f"{inspect.currentframe().f_code.co_name}: {len(seats)}")

part1()
part2()

# part1: 2261
# part2: 