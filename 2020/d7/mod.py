import re

DAY = 'd7'

def parse_line(line):
    pattern = re.compile(r'^(\w+ \w+) bags contain (.+)$')
    match = pattern.match(line)
    if not match:
        return None, []
    
    container = match.group(1)
    contents = match.group(2)
    
    if contents == "no other bags.":
        return container, []
    
    content_pattern = re.compile(r'(\d+) (\w+ \w+) bag')
    content_matches = content_pattern.findall(contents)
    content_list = [(int(count), color) for count, color in content_matches]
    
    return container, content_list

def load_data(file_path):
    with open(file_path, 'r') as file:
        content = file.read()
    lines = content.split('\n')
    ret = {}
    for line in lines:
        container, contents = parse_line(line)
        ret[container] = contents
    return ret

def can_contain_color(bag_rules, current_bag_name, desired_color):
    if current_bag_name not in bag_rules:
        return False
    for _, color in bag_rules[current_bag_name]:
        if color == desired_color or can_contain_color(bag_rules, color, desired_color):
            return True
    return False

def part1():
    bag_rules = load_data(DAY + '\\data_real.txt')
    desired_color = "shiny gold"
    containing_bags = [bag for bag in bag_rules if can_contain_color(bag_rules, bag, desired_color)]
    print(f"part1: {len(containing_bags)}")

def count_bags(bag_rules, current_bag_name):
    if current_bag_name not in bag_rules:
        return 0
    total_count = 1  # Count the current bag itself
    for count, color in bag_rules[current_bag_name]:
        total_count += count * count_bags(bag_rules, color)
    return total_count

def part2():
    groups = load_data(DAY + '\\data_real.txt')
    desired_color = "shiny gold"
    print(f"part2: {count_bags(groups, desired_color) - 1}")  # Subtract 1 to exclude the outermost bag

part1()
part2()

# part1: 274
# part2: 158730