use ::function_name::named;
use regex::Regex;
use std::{cmp, fs, str::FromStr, collections::HashMap};
use utilities::alloc_2d_vec;

const DAY: i32 = 7;

#[derive(Debug, Clone)]
enum Operation {
    SET,
    NOT,
    AND,
    LSHIFT,
    RSHIFT,
    OR,
}

#[derive(Debug, Clone)]
struct Instruction {
    operation: Operation,
    arg1: String,
    arg2: Option<String>,
    dest: String,
}

fn load_data(filename: &str) -> Vec<Instruction> {
    let data = fs::read_to_string(filename).unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();
    let re = Regex::new(r"(.*)\s+->\s+(.*)").unwrap();
    for line in data.lines() {
        for cap in re.captures_iter(line) {
            println!("Matched: {} | {}", &cap[1], &cap[2]);
            let dest: String = cap[2].to_string();
            let split_operation: Vec<&str> = cap[1].split_whitespace().collect();
            let instruction = match split_operation.len() {
                1 => {
                    Instruction {
                        operation: Operation::SET,
                        arg1: split_operation[0].to_string(),
                        arg2: None,
                        dest: dest,
                    }
                }
                2 => {
                    assert!(split_operation[0] == "NOT");
                    Instruction {
                        operation: Operation::NOT,
                        arg1: split_operation[1].to_string(),
                        arg2: None,
                        dest: dest,
                    }
                }
                3 => {
                    let operation = match split_operation[1] {
                        "AND" => Operation::AND,
                        "LSHIFT" => Operation::LSHIFT,
                        "RSHIFT" => Operation::RSHIFT,
                        "OR" => Operation::OR,
                        _ => panic!("Unknown operation: {}", split_operation[1]),
                    };
                    Instruction {
                        operation: operation,
                        arg1: split_operation[0].to_string(),
                        arg2: Some(split_operation[2].to_string()),
                        dest: dest,
                    }
                }
                _ => panic!("Invalid line: {:?}", split_operation),
            };
            instructions.push(instruction);
        }
    }
    instructions
}

fn run_instructions(instructions: &Vec<Instruction>) -> HashMap<String, u16> {
    let mut map: HashMap<String, u16> = HashMap::new();
    for instruction in instructions {
        let arg1 = match instruction.arg1.parse::<u16>() {
            Ok(n) => n,
            Err(_) => match map.get(&instruction.arg1) {
                Some(n) => *n,
                None => panic!("Unknown arg1: {}", instruction.arg1),
            },
        };
        let arg2 = match instruction.arg2 {
            Some(ref arg2) => match arg2.parse::<u16>() {
                Ok(n) => n,
                Err(_) => match map.get(arg2) {
                    Some(n) => *n,
                    None => panic!("Unknown arg2: {}", arg2),
                },
            },
            None => 0,
        };
        let result = match instruction.operation {
            Operation::SET => arg1,
            Operation::NOT => !arg1,
            Operation::AND => arg1 & arg2,
            Operation::LSHIFT => arg1 << arg2,
            Operation::RSHIFT => arg1 >> arg2,
            Operation::OR => arg1 | arg2,
        };
        map.insert(instruction.dest.clone(), result);
        //println!("{:?}", map);
    }
    map
}

#[named]
fn part1() {
    let instructions = load_data(&format!("src\\d{}\\data.txt", DAY));
    let map = run_instructions(&instructions);
        println!("{:?}", map);
        println!("{}: a = {:?}", function_name!(), map["a"]);
}

#[named]
// fn part2() {
//     let instructions = load_data(&format!("src\\d{}\\data_test.txt", DAY));
//     let map = run_instructions(&instructions);
//     println!("{}: {:?}", function_name!(), map);
// }

pub fn run() {
    println!("Day {}:", DAY);
    part1();
    //part2();
}

// part1: 377891
// part2: 14110788
