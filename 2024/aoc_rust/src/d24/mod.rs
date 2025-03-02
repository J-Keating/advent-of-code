use function_name::named;
use itertools::Itertools;
use std::{collections::HashMap, fs, time::Instant};

const DAY: &str = "d24";

#[allow(dead_code)]
mod test_data {
    pub const FILENAME: &str = r"src\d24\data_test2.txt";
}
#[allow(dead_code)]
mod real_data {
    pub const FILENAME: &str = r"src\d24\data.txt";
}

enum Operator {
    AND,
    OR,
    XOR
}

struct Gate {
    a: String,
    b: String,
    op: Operator,
    out: String
}

fn load_data(path: &str) -> (HashMap<String, bool>, Vec<Gate>) {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let (start_values_string, gates_string) = file_contents_as_string
        .split("\r\n\r\n")
        .collect_tuple()
        .unwrap();
    let mut start_values = HashMap::new();
    for line in start_values_string.lines() {
        let (key, value) = line.split_once(":").unwrap();
        start_values.insert(key.trim().to_string(), value.trim().parse::<i32>().unwrap() != 0);
    }
    let mut gates = Vec::new();
    for line in gates_string.lines() {
        let (a, op_str, b, _, out) = line.split_whitespace().collect_tuple().unwrap();
        let op = match op_str {
            "AND" => Operator::AND,
            "OR" => Operator::OR,
            "XOR" => Operator::XOR,
            _ => panic!("Invalid operator"),
        };
        gates.push(Gate {
            a: a.trim().to_string(),
            b: b.trim().to_string(),
            op,
            out: out.trim().to_string(),
        });
    }
    (start_values, gates)
}

fn get_value(values: &mut HashMap<String, bool>, gates: &Vec<Gate>, key: &String) -> bool {
    if let Some(value) = values.get(key) {
        *value
    } else {
        let gate = gates.iter().find(|g| g.out == *key).unwrap();
        let a = get_value(values, gates, &gate.a);
        let b = get_value(values, gates, &gate.b);
        let value = match gate.op {
            Operator::AND => a && b,
            Operator::OR => a || b,
            Operator::XOR => a ^ b,
        };
        values.insert(key.clone(), value);
        value
    }
}

#[named]
fn part1() {
    let start = Instant::now();

    use real_data as data;
    let (start_values, gates) = load_data(data::FILENAME);
    let mut values = start_values.clone();
    let mut all_necessary_outputs = gates.iter().filter(|g| g.out.starts_with("z")).map(|g| g.out.clone()).collect::<Vec<String>>();
    all_necessary_outputs.sort();
    let mut output_as_string = String::new();
    for wire in all_necessary_outputs.iter().rev() {
        let value = get_value(&mut values, &gates, wire);
        output_as_string += if value { "1" } else { "0" };
    }
    let output = u64::from_str_radix(&output_as_string, 2).unwrap();
    
    let duration = start.elapsed();
    println!("{}: {} ==> {} ({}ms)", function_name!(), output_as_string, output, duration.as_millis());
}

#[named]
fn part2() {
    let start = Instant::now();

    use test_data as data;
    let lines = load_data(data::FILENAME);

    let duration = start.elapsed();
    println!("{}: {} ({}ms)", function_name!(), 0, duration.as_millis());
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 315
// part2: 625108891232249
