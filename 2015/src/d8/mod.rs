use ::function_name::named;
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs, vec};

const DAY: i32 = 8;

fn load_data(filename: &str) -> vec::Vec<String> {
    let data = fs::read_to_string(filename).unwrap();
    data.lines().map(|s| s.to_string()).collect()
}

fn count_chars(line: String) -> (usize, usize) {
    let code_len = line.len();
    let mut mem_len = 0;
    let mut pos = 1;
    while (pos < code_len - 1) {
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

#[named]
fn part1() {
    let lines = load_data(&format!("src\\d{}\\data.txt", DAY));
    let mut orig_len_total = 0;
    let mut mem_len_total = 0;
    for line in lines {
        let (orig_len, mem_len) = count_chars(line);
        println!("{}: {} - {} = {}", function_name!(), orig_len, mem_len, orig_len - mem_len);
        orig_len_total += orig_len;
        mem_len_total += mem_len;
    }
    println!("{}: {} - {} = {}", function_name!(), orig_len_total, mem_len_total, orig_len_total - mem_len_total);
}

#[named]
fn part2() {
    println!("{}: a = {:?}", function_name!(), 0);
}

pub fn run() {
    println!("Day {}:", DAY);
    part1();
    part2();
}

// part1: a = 3176
// part2: a = 14710