DAY = 'd8'

def load_data(file_path):
    with open(file_path, 'r') as file:
        content = file.read()
    lines = content.split('\n')
    return [(opcode, int(operand)) for opcode, operand in (line.split(' ') for line in lines)]

def run_simulation(operations, idx_to_flip_nop_jmp): # (is_infite_loop, acc)
    acc = 0
    ip = 0
    visited = [False] * len(operations)
    
    while ip < len(operations) and not visited[ip]:
        visited[ip] = True
        (opcode, operand) = operations[ip]
        if idx_to_flip_nop_jmp and idx_to_flip_nop_jmp == ip:
            assert opcode == "jmp" or opcode == "nop"
            opcode = "nop" if opcode == "jmp" else "jmp"

        match opcode: 
            case "nop":
                ip += 1
            case  "acc":
                acc += operand
                ip += 1
            case "jmp":
                ip += operand
            case _:
                assert False, ("Unknown Operation: " + operations[ip][0])
    return (ip < len(operations), acc)

def part1():
    operations = load_data(DAY + '\\data_real.txt')
    is_infinite_loop, acc = run_simulation(operations, None)
    assert is_infinite_loop
    print(f"part1: {acc}")

def part2():
    operations = load_data(DAY + '\\data_real.txt')
    flip_indicies = [idx for idx, op in enumerate(operations) if op[0] == "nop" or op[0] == "jmp"]
    for flip_idx in flip_indicies:
        is_infinite_loop, acc = run_simulation(operations, flip_idx)
        if not is_infinite_loop:
            break
    assert not is_infinite_loop
    print(f"part2: {acc}")

part1()
part2()

# part1: 1727
# part2: 552