use ::function_name::named;
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs, vec};

const DAY: i32 = 8;

fn load_data(filename: &str) -> vec::Vec<String> {
    let data = fs::read_to_string(filename).unwrap();
    data.lines().map(|s| s.to_string()).collect()
}

fn count_chars(line: &String) -> (usize, usize) {
    let code_len = line.len();
    let mut mem_len = 0;
    let mut pos = 1;
    while pos < code_len - 1 {
        match line.chars().nth(pos).unwrap() {
            '\\' => {
                match line.chars().nth(pos + 1).unwrap() {
                    '\\' | '"' => {
                        pos += 1;
                    }
                    'x' => {
                        pos += 3;
                    }
                    _ => panic!("Unknown escape sequence: {}", line),
                }
            }
            _ => {}
        }
        mem_len += 1;
        pos += 1;
    }
    (code_len, mem_len)
}

fn encode_string(line: &String) -> String {
    let mut ret = String::new();
    ret.push('"');
    for c in line.chars() {
        match c {
            '\\' => {
                ret.push_str("\\\\");
            }
            '"' => {
                ret.push_str("\\\"");
            }
            _ => {
                ret.push(c);
            }
        }
    }
    ret.push('"');
    ret
}

#[named]
fn part1() {
    let lines = load_data(&format!("src\\d{}\\data.txt", DAY));
    let res = lines.iter().map(| f | count_chars(f)).fold(0, |acc, (a, b)| acc + a - b);
    println!("{}: {}", function_name!(), res);
}

#[named]
fn part2() {
    let lines = load_data(&format!("src\\d{}\\data.txt", DAY));
    let res = lines.iter().map(| f | encode_string(f).len() - f.len()).sum::<usize>();
    println!("{}: {}", function_name!(), res);
}

pub fn run() {
    println!("Day {}:", DAY);
    part1();
    part2();
}

// part1: 1333
// part2: 2046