use core::panic;
use function_name::named;
use itertools::Itertools;
use std::fs;

const DAY: &str = "d17";

mod test_data {
    pub const FILENAME: &str = r"src\d17\data_test2.txt";
}
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

#[named]
fn part2() {
    use real_data as data;
    let (mut c, program) = load_data(data::FILENAME);
    let mut output = Vec::<i8>::new();

    for i in 0..=60 {
        c.a = i;
        c.b = 0;
        c.c = 0;
        c.ip = 0;
        output.clear();
        while c.ip < program.len() {
            run_one_instruction(&mut c, &program, &mut output);
        }
        println!("{}: {}", i, output.iter().join(","));
    }
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 1436690
// part2: 1482350
// part2: 1482350
