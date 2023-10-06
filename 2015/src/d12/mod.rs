use ::function_name::named;
use std::{fs, vec};
use json::JsonValue;

const DAY: i32 = 12;

fn get_all_numbers_in_json_value(value: &JsonValue, numbers: &mut Vec<i32>, blocking_value: &str) {
    match value {
        JsonValue::Number(n) => numbers.push(f32::from(*n) as i32),
        JsonValue::Object(o) => {
            if o.iter().any(|(_, v)| v == blocking_value) {
                return;
            }
            o.iter().for_each(|(_, v)| get_all_numbers_in_json_value(v, numbers, blocking_value));
        }
        JsonValue::Array(a) => { a.iter().for_each(| m: &JsonValue | get_all_numbers_in_json_value(m, numbers, blocking_value)); },
        _ => {}
    }
}

fn get_all_numbers_in_json_string(json_string: &str, blocking_value: &str) -> Vec<i32> {
    let mut numbers = vec![];
    let root: JsonValue = json::parse(json_string).unwrap();
    get_all_numbers_in_json_value(&root, &mut numbers, blocking_value);
    numbers
}

#[test]
fn json_test() {
    let x = get_all_numbers_in_json_string(r#"
    {
        "a": 1,
        "b": 2,
        "c": {
            "d": 3,
            "e": 4
        }
    }"#, "");
    assert!(x.iter().sum::<i32>() == 10);
}

#[named]
fn part1() {
    let json = fs::read_to_string(&format!("src\\d{}\\data.txt", DAY)).unwrap();
    let numbers = get_all_numbers_in_json_string(&json, "");
    println!("{}: {}", function_name!(), numbers.iter().sum::<i32>());
}

#[named]
fn part2() {
    let json = fs::read_to_string(&format!("src\\d{}\\data.txt", DAY)).unwrap();
    let numbers = get_all_numbers_in_json_string(&json, "red");
    println!("{}: {}", function_name!(), numbers.iter().sum::<i32>());
}

pub fn run() {
    println!("Day {}:", DAY);
    part1();
    part2();
}

// part1: 191164
// part2: 909