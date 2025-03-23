from email.policy import default
from itertools import accumulate
import re

DAY = 'd8'

def load_data(file_path):
    with open(file_path, 'r') as file:
        content = file.read()
    lines = content.split('\n')
    return [(opcode, int(operand)) for opcode, operand in (line.split(' ') for line in lines)]

def part1():
    operations = load_data(DAY + '\\data_real.txt')
    accumulator = 0
    ip = 0
    visited = [False] * len(operations)
    
    while ip < len(operations) and not visited[ip]:
        visited[ip] = True
        match operations[ip][0]: 
            case "nop":
                ip += 1
            case  "acc":
                accumulator += operations[ip][1]
                ip += 1
            case "jmp":
                ip += operations[ip][1]
            case _:
                assert False, ("Unknown Operation: " + operations[ip][0])

    assert ip < len(operations)

    print(f"part1: {accumulator}")

def part2():
    operations = load_data(DAY + '\\data_test.txt')
    print(f"part2: {len(operations)}")

part1()
part2()

# part1: 1727
# part2: 