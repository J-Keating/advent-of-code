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

#[derive(Debug)]
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

fn load_data(path: &str, output_swap_fn: fn(&String) -> String) -> (HashMap<String, bool>, Vec<Gate>) {
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
            out: output_swap_fn(&out.trim().to_string()),
        });
    }
    (start_values, gates)
}

fn get_value(values: &mut HashMap<String, bool>, gates: &Vec<Gate>, key: &String, verbose: bool) -> bool {
    if let Some(value) = values.get(key) {
        if verbose && (key.starts_with("x") || key.starts_with("y")) {
            println!("{}: {}", key, if *value { "1" } else { "0" });
        }
        *value
    } else {
        let gate = gates.iter().find(|g| g.out == *key).unwrap();
        let a = get_value(values, gates, &gate.a, verbose);
        let b = get_value(values, gates, &gate.b, verbose);
        let value = match gate.op {
            Operator::AND => a && b,
            Operator::OR => a || b,
            Operator::XOR => a ^ b,
        };
        values.insert(key.clone(), value);
        if verbose {
            println!("{}: {}({}) {:?} {}({}) ==> {}", key,
                gate.a, if a { "1" } else { "0" }, 
                gate.op,
                gate.b, if b { "1" } else { "0" },
                if value { "1" } else { "0" });
        }
        value
    }
}

#[named]
fn part1() {
    let start = Instant::now();

    use real_data as data;
    let (start_values, gates) = load_data(data::FILENAME, |s| s.clone());
    let mut values = start_values.clone();
    let mut all_necessary_outputs = gates.iter().filter(|g| g.out.starts_with("z")).map(|g| g.out.clone()).collect::<Vec<String>>();
    all_necessary_outputs.sort();
    let mut output_as_string = String::new();
    for wire in all_necessary_outputs.iter().rev() {
        let value = get_value(&mut values, &gates, wire, false);
        output_as_string += if value { "1" } else { "0" };
    }
    let output = u64::from_str_radix(&output_as_string, 2).unwrap();

    let duration = start.elapsed();
    println!("{}: {} ==> {} ({}ms)", function_name!(), output_as_string, output, duration.as_millis());
}

fn run_simulation(gates: &Vec<Gate>, all_necessary_outputs: &Vec<String>, a: u64, b: u64, verbose: bool) -> u64 {
    let mut start_values = HashMap::new();
    let a_str = format!("{:0>46b}", a);
    let b_str = format!("{:0>46b}", b);
    for (i, c) in a_str.chars().rev().enumerate() {
        start_values.insert(format!("x{:02}", i), c == '1');
    }
    for (i, c) in b_str.chars().rev().enumerate() {
        start_values.insert(format!("y{:02}", i), c == '1');
    }
    let mut values = start_values.clone();
    let output_as_string = all_necessary_outputs.iter().rev().map(|wire| {
        let value = get_value(&mut values, gates, wire, verbose);
        if value {
            "1"
        } else {
            "0"
        }
    }).collect::<String>();
    // if verbose {
    //     values.iter().filter(|kv| !kv.0.starts_with("x") && !kv.0.starts_with("y")).for_each(|(k, v)| {
    //         println!("{}: {}", k, if *v { "1" } else { "0" });
    //     });
    // }
    u64::from_str_radix(&output_as_string, 2).unwrap()
}

#[named]
fn part2() {
    let start = Instant::now();

    let remap_fn = |s: &String| {
        match s.as_str() {
            "z07" => "shj".to_string(),
            "shj" => "z07".to_string(),
            "wkb" => "tpk".to_string(),
            "tpk" => "wkb".to_string(),
            "pfn" => "z23".to_string(),
            "z23" => "pfn".to_string(),
            "kcd" => "z27".to_string(),
            "z27" => "kcd".to_string(),
            _ => s.clone()
        }
    };

    use real_data as data;
    let (_, gates) = load_data(data::FILENAME, remap_fn);
    let mut all_necessary_outputs = gates.iter().filter(|g| g.out.starts_with("z")).map(|g| g.out.clone()).collect::<Vec<String>>();
    all_necessary_outputs.sort();

    let run_test_case = |a: u64, b: u64| {
        let answer = run_simulation(&gates, &all_necessary_outputs, a, b, false);
        if answer != a + b {
            println!("  {:0>46b}\n+ {:0>46b}\n= {:0>46b}\nx {:0>46b}\n. {:0>46b}\n", a, b, a + b, answer, (a + b) ^ answer);
            // let mistake_bits_as_int = (a + b) ^ answer;
            // let mistake_bits = (0..45).filter(|i| (mistake_bits_as_int >> i) & 1 == 1).map(|i| format!("z{:02}", i)).collect::<Vec<String>>();
            let mistake_bits = vec!["z27".to_string()];
            run_simulation(&gates, &mistake_bits, a, b, true);            
        }
    };
    run_test_case(0, 0);
    for i in 0..45 {
        run_test_case(0, 1 << i);
        run_test_case(1 << i, 0);
        run_test_case(1 << i, 1 << i);
    }
    let answer = "kcd,pfn,shj,tpk,wkb,z07,z23,z27";

    let duration = start.elapsed();
    println!("{}: {} ({}ms)", function_name!(), answer, duration.as_millis());
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: part1: 1101000110000001000100111011111110001101001110 ==> 57588078076750 (2ms)
// part2: part2: kcd,pfn,shj,tpk,wkb,z07,z23,z27 (227ms)
