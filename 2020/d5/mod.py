DAY = 'd5'

fields = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
    "cid"
]

def load_data(file_path):
    with open(file_path, 'r') as file:
        content = file.read()
    return content.split('\n')

def decode_seat_to_row_col(seat_code):
    row_code = seat_code[:7]
    col_code = seat_code[7:]
    
    row = int(row_code.replace('F', '0').replace('B', '1'), 2)
    col = int(col_code.replace('L', '0').replace('R', '1'), 2)
    
    return row, col

def decode_seat(seat_code):
    row, col = decode_seat_to_row_col(seat_code)
    return row * 8 + col

def part1():
    data = load_data(DAY + '\\data_real.txt')
    seat_ids = [decode_seat(seat) for seat in data]
    print(f"part1: {max(seat_ids)}")

def part2():
    data = load_data(DAY + '\\data_real.txt')
    seat_ids = [decode_seat(seat) for seat in data]
    seat_ids.sort()
    for first, second in zip(seat_ids, seat_ids[1:]):
        if second - first > 1:
            print(f"part1: {first + 1}")

part1()
part2()

# part1: 904
# part1: 669