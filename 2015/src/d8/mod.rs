use ::function_name::named;
use std::{ fs, vec};

const DAY: i32 = 8;

fn load_data(filename: &str) -> vec::Vec<String> {
    let data = fs::read_to_string(filename).unwrap();
    data.lines().map(|s| s.to_string()).collect()
}

fn count_chars(line: &String) -> (usize, usize) {
    let mut mem_len = 0;
    let mut trimmed_buffer = line.chars().skip(1).take(line.len() - 2);
    loop {
        match trimmed_buffer.next() {
            None => break,
            Some(c) => {
                if c == '\\' {
                    match trimmed_buffer.next().unwrap() {
                        '\\' | '"' => {}
                        'x' => {
                            trimmed_buffer.next();
                            trimmed_buffer.next();
                        }
                        _ => panic!("Unknown escape sequence: {}", line),
                    }
                }
                mem_len += 1;
            }
        }
    }
    (line.len(), mem_len)
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
    let res = lines.iter().map(| f | count_chars(f)).map(| (code_len, mem_len) | code_len - mem_len).sum::<usize>();
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