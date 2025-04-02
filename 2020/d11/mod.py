import inspect
import time

DAY = 'd11'
FULL_SEAT = '#'
EMPTY_SEAT = 'L'
FLOOR = '.'

class TEST_DATA:
    FILE_NAME = 'data_test.txt'

class REAL_DATA:
    FILE_NAME = 'data_real.txt'

NEIGHBORS = [   (-1, -1), (0, -1), (1, -1),
                (-1,  0),          (1,  0),
                (-1,  1), (0,  1), (1,  1)]

def load_data(file_path):
    with open(file_path, 'r') as file:
        return [list(line.strip()) for line in file]
    
def print_array(array):
    for row in array:
        print(''.join(row))
    print()

def in_bounds(array, pos):
    return pos[0] >= 0 and pos[0] < len(array) and pos[1] >= 0 and pos[1] < len(array[0])

def count_adjacent_occupied(seats, pos):
    occupied = [n for n in NEIGHBORS if in_bounds(seats, (pos[0] + n[0],pos[1] + n[1])) and seats[pos[0] + n[0]][pos[1] + n[1]] == FULL_SEAT]
    return len(occupied)

def new_seat_state_p1(seats, pos):
    match seats[pos[0]][pos[1]]:
        case x if x == FLOOR:
            return None
        case x if x == EMPTY_SEAT:
            return FULL_SEAT if count_adjacent_occupied(seats, pos) == 0 else None
        case x if x == FULL_SEAT:
            return EMPTY_SEAT if count_adjacent_occupied(seats, pos) >= 4 else None

def part1():
    start_time = time.perf_counter()
    data = REAL_DATA
    new_seats = load_data(DAY + '\\' + data.FILE_NAME)
    any_changed = True
    while any_changed:
        seats = [row.copy() for row in new_seats]
        any_changed = False
        for r in range(0, len(seats)):
            for c in range(0, len(seats[0])):
                new_state = new_seat_state_p1(seats, (r,c))
                if new_state:
                    new_seats[r][c] = new_state
                    any_changed = True
        #print_array(new_seats)
    occupied = sum(len([s for s in row if s == FULL_SEAT]) for row in seats)
    elapsed_time = time.perf_counter() - start_time
    print(f"{inspect.currentframe().f_code.co_name} ({elapsed_time:.2f} sec) : {occupied}")

def count_visible_seats(seats, pos):
    visible_seat_count = 0
    for n in NEIGHBORS:
        p = (pos[0] + n[0], pos[1] + n[1])
        while in_bounds(seats, p):
            match seats[p[0]][p[1]]:
                case x if x == FULL_SEAT:
                    visible_seat_count += 1
                    break
                case x if x == EMPTY_SEAT:
                    break
            p = (p[0] + n[0], p[1] + n[1])
    return visible_seat_count

def new_seat_state_p2(seats, pos):
    match seats[pos[0]][pos[1]]:
        case x if x == FLOOR:
            return None
        case x if x == EMPTY_SEAT:
            return FULL_SEAT if count_visible_seats(seats, pos) == 0 else None
        case x if x == FULL_SEAT:
            return EMPTY_SEAT if count_visible_seats(seats, pos) >= 5 else None

def part2():
    start_time = time.perf_counter()
    data = REAL_DATA
    new_seats = load_data(DAY + '\\' + data.FILE_NAME)
    any_changed = True
    while any_changed:
        seats = [row.copy() for row in new_seats]
        any_changed = False
        for r in range(0, len(seats)):
            for c in range(0, len(seats[0])):
                new_state = new_seat_state_p2(seats, (r,c))
                if new_state:
                    new_seats[r][c] = new_state
                    any_changed = True
        #print_array(new_seats)
    occupied = sum(len([s for s in row if s == FULL_SEAT]) for row in seats)
    elapsed_time = time.perf_counter() - start_time
    print(f"{inspect.currentframe().f_code.co_name} ({elapsed_time:.2f} sec) : {occupied}")

part1()
part2()

# part1: 2261
# part2: 