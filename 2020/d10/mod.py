
import inspect

DAY = 'd10'

class TEST_DATA:
    FILE_NAME = 'data_test2.txt'

class REAL_DATA:
    FILE_NAME = 'data_real.txt'

def load_data(file_path):
    with open(file_path, 'r') as file:
        content = file.read()
    return [int(line) for line in content.split('\n')]

def part1():
    data = REAL_DATA
    nums = load_data(DAY + '\\' + data.FILE_NAME)
    nums.append(0)
    nums.sort()
    nums.append(nums[len(nums)-1] + 3)
    counts = [0] * 4
    for i, j in zip(nums, nums[1:]):
        diff = j - i
        assert(diff < 4)
        counts[diff] += 1
    # print(f"{counts}")
    print(f"{inspect.currentframe().f_code.co_name}: {counts[1] * counts[3]}")

def part2():
    data = TEST_DATA
    nums = load_data(DAY + '\\' + data.FILE_NAME)
    print(f"{inspect.currentframe().f_code.co_name}: {len(nums)}")

part1()
part2()

# part1: 2030
# part2: 