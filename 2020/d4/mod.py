from functools import reduce
from operator import mul
from typing import List, Mapping

DAY = 'd4'

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

def load_data(file_path) -> List[dict]:
    with open(file_path, 'r') as file:
        content = file.read()
    passports = content.split('\n\n')
    ret = []
    for passport in passports:
        fields = passport.split()
        passport_dict = {}
        for field in fields:
            key, value = field.split(':')
            passport_dict[key] = value
        ret.append(passport_dict)
    return ret

def part1():
    data = load_data(DAY + '\\data_test.txt')
    valid_passports = []
    for passport in data:
        if all(field in passport for field in fields[:-1]):
            valid_passports.append(passport)
    print(f"part1: {valid_passports}")

def part2():
    data = load_data(DAY + '\\data_test.txt')
    print(f"part1: {data}")    

part1()
part2()

# part1: 195
# part1: 3772314000