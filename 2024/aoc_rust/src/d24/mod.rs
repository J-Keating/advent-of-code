use function_name::named;
use itertools::Itertools;
use std::{collections::HashMap, fs, time::Instant};

const DAY: &str = "d24";

#[allow(dead_code)]
mod test_data {
    pub const FILENAME: &str = r"src\d24\data_test.txt";
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

#[named]
fn part1() {
    let start = Instant::now();

    use test_data as data;
    let (start_values, gates) = load_data(data::FILENAME);

    let duration = start.elapsed();
    println!("{}: {} ({}ms)", function_name!(), start_values.len() + gates.len(), duration.as_millis());
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
