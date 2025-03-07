import re
from dataclasses import dataclass
from typing import List, Callable

DAY = 'd2'

@dataclass
class Policy:
    low: int
    high: int
    char: str
    string: str

def load_data(file_path) -> List[Policy]:
    pattern = re.compile(r'(\d+)-(\d+) (\w): (\w+)')
    policies = []
    with open(file_path, 'r') as lines:
        for line in lines:
            match = pattern.match(line)
            if match:
                low, high, char, string = match.groups()
                policies.append(Policy(int(low), int(high), char, string))
    return policies

def is_valid_p1(policy: Policy) -> bool:
    count = policy.string.count(policy.char)
    return policy.low <= count <= policy.high

def is_valid_p2(policy: Policy) -> bool:
    first = policy.string[policy.low - 1] == policy.char
    second = policy.string[policy.high - 1] == policy.char
    return first != second

def count_is_valid(test_fn: Callable[[Policy], bool]) -> int:
    data = load_data(DAY + '\\data_real.txt')
    return len([policy for policy in data if test_fn(policy)])

def part1():
    print(f"part1: {count_is_valid(is_valid_p1)}")

def part2():
    print(f"part2: {count_is_valid(is_valid_p2)}")

part1()
part2()

# part1: 548
# part2: 502