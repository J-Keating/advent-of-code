use core::panic;
use function_name::named;
use itertools::Itertools;
use std::fs;

const DAY: &str = "d17";

#[allow(dead_code)]
mod test_data {
    pub const FILENAME: &str = r"src\d17\data_test2.txt";
}
#[allow(dead_code)]
mod real_data {
    pub const FILENAME: &str = r"src\d17\data.txt";
}

struct Computer {
    ip: usize,
    a: i64,
    b: i64,
    c: i64,
}

fn load_data(path: &str) -> (Computer, Vec<i8>) {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let (register_string, mut program_string) = file_contents_as_string
        .split("\r\n\r\n")
        .collect_tuple()
        .unwrap();
    let registers_lines = register_string.lines().collect::<Vec<&str>>();
    assert!(registers_lines.len() == 3);
    let computer = Computer {
        ip: 0,
        a: registers_lines[0]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap(),
        b: registers_lines[1]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap(),
        c: registers_lines[2]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap(),
    };
    program_string = program_string
        .split_whitespace()
        .into_iter()
        .last()
        .unwrap();
    (
        computer,
        program_string
            .split(",")
            .map(|s| s.parse::<i8>().unwrap())
            .collect(),
    )
}

fn get_combo_operand(c: &Computer, literal_operand: i64) -> i64 {
    match literal_operand {
        x @ 0..=3 => x,
        4 => c.a,
        5 => c.b,
        6 => c.c,
        _ => panic!("Invalid literal operand: {}", literal_operand),
    }
}

fn run_one_instruction(c: &mut Computer, program: &Vec<i8>, output: &mut Vec<i8>) {
    let instruction = program[c.ip];
    c.ip += 1;
    let literal_operand = program[c.ip] as i64;
    c.ip += 1;
    match instruction {
        0 => {
            // adv:  The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand
            // The result of the division operation is truncated to an integer and then written to the A register.
            c.a = c.a / 2_i64.pow(get_combo_operand(c, literal_operand) as u32);
        }
        1 => {
            // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
            c.b = c.b ^ literal_operand;
        }
        2 => {
            // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
            c.b = get_combo_operand(c, literal_operand) % 8;
        }
        3 => {
            // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the
            // instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
            if c.a != 0 {
                c.ip = literal_operand as usize;
            }
        }
        4 => {
            // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
            c.b = c.b ^ c.c;
        }
        5 => {
            // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
            output.push((get_combo_operand(c, literal_operand) % 8) as i8);
        }
        6 => {
            // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
            c.b = c.a / 2_i64.pow(get_combo_operand(c, literal_operand) as u32);
        }
        7 => {
            // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
            c.c = c.a / 2_i64.pow(get_combo_operand(c, literal_operand) as u32);
        }
        _ => panic!("Unknown instruction: {}", instruction),
    }
}

#[named]
fn part1() {
    use real_data as data;
    let (mut c, program) = load_data(data::FILENAME);
    let mut output = Vec::<i8>::new();

    while c.ip < program.len() {
        run_one_instruction(&mut c, &program, &mut output);
    }
    println!("{}: {}", function_name!(), output.iter().join(","));
}

fn get_output_from_starting_a_value(program: &Vec<i8>, a: i64, output: &mut Vec<i8>) {
    let mut c = Computer {
        ip: 0,
        a: a,
        b: 0,
        c: 0,
    };
    output.clear();
    while c.ip < program.len() {
        run_one_instruction(&mut c, &program, output);
    }
}

#[allow(dead_code)]
fn test_one_a_value(_: &mut Computer, program: &Vec<i8>, a: i64) {
    let mut output = Vec::<i8>::new();
    get_output_from_starting_a_value(program, a, &mut output);
    println!("{:6o}: {}", a, output.iter().join(","));
}

#[test]
fn test() {
    use real_data as data;
    let (mut c, program) = load_data(data::FILENAME);
    test_one_a_value(&mut c, &program, 0o770717);
    test_one_a_value(&mut c, &program, 0o771717);
    test_one_a_value(&mut c, &program, 0o772717);
    test_one_a_value(&mut c, &program, 0o773717);
    test_one_a_value(&mut c, &program, 0o774717);
    test_one_a_value(&mut c, &program, 0o775717);
    test_one_a_value(&mut c, &program, 0o776717);
    test_one_a_value(&mut c, &program, 0o777717);
}

#[test]
fn test2() {
    use real_data as data;
    let (mut c, program) = load_data(data::FILENAME);
    // test_one_a_value(&mut c, &program, 0o7200000000000000);
    // test_one_a_value(&mut c, &program, 0o7210000000000000);
    // test_one_a_value(&mut c, &program, 0o7220000000000000);
    // test_one_a_value(&mut c, &program, 0o7230000000000000);
    // test_one_a_value(&mut c, &program, 0o7240000000000000);
    // test_one_a_value(&mut c, &program, 0o7250000000000000);
    // test_one_a_value(&mut c, &program, 0o7260000000000000);
    // test_one_a_value(&mut c, &program, 0o7270000000000000);

    // test_one_a_value(&mut c, &program, 0o7400000000000000);
    // test_one_a_value(&mut c, &program, 0o7410000000000000);
    // test_one_a_value(&mut c, &program, 0o7420000000000000);
    // test_one_a_value(&mut c, &program, 0o7430000000000000);
    // test_one_a_value(&mut c, &program, 0o7440000000000000);
    // test_one_a_value(&mut c, &program, 0o7450000000000000);
    // test_one_a_value(&mut c, &program, 0o7460000000000000);
    // test_one_a_value(&mut c, &program, 0o7470000000000000);

    for i in 0..=7 {
        test_one_a_value(&mut c, &program, 0o7260000000000000 | i << (12 * 3));
    }
    // 7260000000000000: 0,0,0,0,0,0,0,0,0,0,0,3,5,5,3,0
    for i in 0..=7 {
        test_one_a_value(&mut c, &program, 0o7260000000000000 | i << (11 * 3));
    }
    // 7260100000000000: 0,0,0,0,0,0,0,0,4,0,0,7,5,5,3,0
    for i in 0..=7 {
        test_one_a_value(&mut c, &program, 0o7260100000000000 | i << (10 * 3));
    }
    // 7260110000000000: 0,0,0,0,0,0,0,4,4,0,1,7,5,5,3,0
    for i in 0..=7 {
        test_one_a_value(&mut c, &program, 0o7260110000000000 | i << (9 * 3));
    }
    // None ends with 4,1,7,5,5,3,0

    //test_one_a_value(&mut c, &program, 0o7260000000000000);
}

fn test_octets(program: &Vec<i8>, so_far: i64, octet: i64) -> bool {
    let mut output = Vec::<i8>::new();
    for i in 0..=7 {
        let a = so_far | i << (octet * 3);
        let start_index = octet as usize;
        get_output_from_starting_a_value(program, a, &mut output);
        if program.len() == output.len()
            && program[start_index..output.len()] == output[start_index..output.len()]
        {
            //println!("{:16o}: {}", a, output.iter().join(","));
            if octet > 0 {
                if test_octets(program, a, octet - 1) {
                    return true;
                }
            } else {
                println!("Found!: {} (0o{:o}): {}", a, a, output.iter().join(","));
                return true;
            }
        }
    }
    false
}

//#[named]
fn part2() {
    use real_data as data;
    let (_, program) = load_data(data::FILENAME);
    test_octets(&program, 0, program.len() as i64 - 1);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 2,1,0,1,7,2,5,0,3
// Found!: 267265166222235 (0o7461160522621633): 2,4,1,7,7,5,0,3,4,4,1,7,5,5,3,0
