DAY = 'd6'

def load_data(file_path):
    with open(file_path, 'r') as file:
        content = file.read()
    groups = content.split('\n\n')
    return [group.split('\n') for group in groups]

def char_to_index(char):
    return ord(char) - ord('a')

def part1():
    groups = load_data(DAY + '\\data_real.txt')
    sum = 0
    for group in groups:
        char_flags = [False] * 26
        for person in group:
            for char in person:
                char_flags[char_to_index(char)] = True
        sum += len([char for char in char_flags if char])
    print(f"part1: {sum}")

def part2():
    groups = load_data(DAY + '\\data_real.txt')
    sum = 0
    for group in groups:
        char_counts = [0] * 26
        for person in group:
            for char in person:
                char_counts[char_to_index(char)] += 1
        sum += len([count for count in char_counts if count == len(group)])
    print(f"part2: {sum}")

part1()
part2()

# part1: 6170
# part2: 2947