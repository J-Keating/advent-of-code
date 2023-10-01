use ::function_name::named;
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs};

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
}

fn load_data(filename: &str) -> HashMap<String, Instruction> {
    let data = fs::read_to_string(filename).unwrap();
    let mut connections: HashMap<String, Instruction> = HashMap::new();
    let re = Regex::new(r"(.*)\s+->\s+(.*)").unwrap();
    for line in data.lines() {
        for cap in re.captures_iter(line) {
            let dest: String = cap[2].to_string();
            let split_operation: Vec<&str> = cap[1].split_whitespace().collect();
            let instruction = match split_operation.len() {
                1 => Instruction {
                    operation: Operation::SET,
                    arg1: split_operation[0].to_string(),
                    arg2: None,
                },
                2 => {
                    assert!(split_operation[0] == "NOT");
                    Instruction {
                        operation: Operation::NOT,
                        arg1: split_operation[1].to_string(),
                        arg2: None,
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
                    }
                }
                _ => panic!("Invalid line: {:?}", split_operation),
            };
            connections.insert(dest, instruction);
        }
    }
    connections
}

fn evaluate_signal(connections: &HashMap<String, Instruction>, wire: &String, signal_cache: &mut HashMap<String, u16>) -> u16 {
    if signal_cache.contains_key(wire) {
        return *signal_cache.get(wire).unwrap();
    }
    let instruction: &Instruction = connections.get(wire).unwrap();
    let arg1 = match instruction.arg1.parse::<u16>() {
        Ok(n) => n,
        Err(_) => evaluate_signal(connections, &instruction.arg1, signal_cache),
    };
    let arg2 = match instruction.arg2 {
        Some(ref arg2) => match arg2.parse::<u16>() {
            Ok(n) => n,
            Err(_) => evaluate_signal(connections, arg2, signal_cache),
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
    signal_cache.insert(wire.to_string(), result);
    result
}

// #[named]
// fn part1_test() {
//     let connections = load_data(&format!("src\\d{}\\data_test.txt", DAY));
//     let mut signal_cache: HashMap<String, u16> = HashMap::new();
//     connections.keys().sorted().for_each(|k| {
//         let res = evaluate_signal(&connections, k, &mut signal_cache);
//         println!("{}: {} = {}", function_name!(), k, res);
//     });
// }

#[named]
fn part1() {
    let connections = load_data(&format!("src\\d{}\\data.txt", DAY));
    let mut signal_cache: HashMap<String, u16> = HashMap::new();
    let res = evaluate_signal(&connections, &"a".to_string(), &mut signal_cache);
    println!("{}: a = {:?}", function_name!(), res);
}

#[named]
fn part2() {
    // part1: a = 3176
    let connections = load_data(&format!("src\\d{}\\data.txt", DAY));
    let mut signal_cache: HashMap<String, u16> = HashMap::new();
    signal_cache.insert("b".to_string(), 3176);
    let res = evaluate_signal(&connections, &"a".to_string(), &mut signal_cache);
    println!("{}: a = {:?}", function_name!(), res);
}

pub fn run() {
    println!("Day {}:", DAY);
    //part1_test();
    part1();
    part2();
}

// part1: a = 3176
// part2: a = 14710
