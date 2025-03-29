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
    #print(f"{counts}")
    print(f"{inspect.currentframe().f_code.co_name}: {counts[1] * counts[3]}")

def part2():
    data = REAL_DATA
    nums = load_data(DAY + '\\' + data.FILE_NAME)
    nums.append(0)
    nums.sort()
    nums.append(nums[len(nums)-1] + 3)
    diffs = [j - i for i, j in zip(nums, nums[1:])]
    runs = []
    curr_run_len = 0
    curr_run_value = -1
    for diff in diffs:
        if diff == curr_run_value:
            curr_run_len += 1
        else:
            runs.append((curr_run_value, curr_run_len))
            curr_run_len = 1
            curr_run_value = diff
    #print(f"{diffs}")
    #print(f"{runs}")
    single_step_run_lengths = [x[1] for x in runs if x[0] == 1]
    #print(f"{single_step_run_lengths}")
    res = 1
    for run_length in single_step_run_lengths:
        match run_length:
            case 1:
                pass
            case 2:
                res *= 2
            case 3:
                res *= 4
            case 4:
                res *= 7
            case _:
                assert(f"Unhandled length {run_length}")
    print(f"{inspect.currentframe().f_code.co_name}: {res}")

part1()
part2()

# part1: 2030
# part2: 42313823813632