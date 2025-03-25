import itertools

DAY = 'd9'

class TEST_DATA:
    WINDOW_SIZE = 5
    FILE_NAME = 'data_test.txt'

class REAL_DATA:
    WINDOW_SIZE = 25
    FILE_NAME = 'data_real.txt'

def load_data(file_path):
    with open(file_path, 'r') as file:
        content = file.read()
    return [int(line) for line in content.split('\n')]

def part1():
    data = REAL_DATA
    nums = load_data(DAY + '\\' + data.FILE_NAME)
    window_size = data.WINDOW_SIZE
    curr = window_size + 1
    for curr in range(window_size + 1, len(nums)):
        to_find = nums[curr]
        tuples = list(itertools.combinations(nums[curr-1-window_size:curr], 2))
        valid_tuples = [t for t in tuples if t[0] + t[1] == to_find]
        valid = len(valid_tuples) > 0
        if not valid:
            break
    print(f"part1: {to_find}")

def part2():
    target_num = 18272118
    data = REAL_DATA
    nums = load_data(DAY + '\\' + data.FILE_NAME)
    target_num_index = nums.index(target_num)
    size = 2
    answer = 0
    while answer == 0:
        for start in range(0, target_num_index - size):
            test_range = nums[start : start + size]
            if sum(test_range) == target_num:
                answer = min(test_range) + max(test_range)
                break
        size += 1
    print(f"part2: {answer}")

part1()
part2()

# part1: 18272118
# part2: 2186361