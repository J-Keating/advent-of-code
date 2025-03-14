from typing import List

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
    data = load_data(DAY + '\\data_real.txt')
    valid_passports = []
    for passport in data:
        if all(field in passport for field in fields[:-1]):
            valid_passports.append(passport)
    print(f"part1: {len(valid_passports)}")

def validate_field(field: str, value: str) -> bool:
    match field:
        case "byr":
            return 1920 <= int(value) <= 2002
        case "iyr":
            return 2010 <= int(value) <= 2020
        case "eyr":
            return 2020 <= int(value) <= 2030
        case "hgt":
            if value.endswith("cm"):
                return 150 <= int(value[:-2]) <= 193
            elif value.endswith("in"):
                return 59 <= int(value[:-2]) <= 76
            else:
                return False
        case "hcl":
            return len(value) == 7 and value[0] == '#' and all(c in '0123456789abcdef' for c in value[1:])
        case "ecl":
            return value in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        case "pid":
            return len(value) == 9 and value.isdigit()
        case _:
            return True
 
def part2():
    data = load_data(DAY + '\\data_real.txt')
    # def data_filter(passport):
    #     all(validate_field(field, passport[field]) for field in fields[:-1] if field in passport)
    valid_passports = []
    for passport in data:
        if all((field in passport and validate_field(field, passport[field])) for field in fields[:-1]):
            valid_passports.append(passport)
    print(f"part1: {len(valid_passports)}")

part1()
part2()

# part1: 195
# part1: 3772314000