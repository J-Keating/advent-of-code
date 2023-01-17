use std::{fs, str::FromStr, cmp};
use ::function_name::named;
use regex::Regex;
use utilities::{alloc_2d_vec};

const DAY: i32 = 7;

#[test]
fn test_regex() {
    // turn on 887,9 through 959,629
    let re = Regex::new(r"(turn on|turn off|toggle)\s+(\d+),(\d+) through (\d+),(\d+).*").unwrap();
    let text = "turn on 454,398 through 844,448";
    for cap in re.captures_iter(text) {
        println!("Matched: {} {} {} {} {}", &cap[1], &cap[2], &cap[3], &cap[4], &cap[5]);
    }
}

#[derive(Debug, Copy, Clone)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    action: Action,
    start: (usize, usize),
    end: (usize, usize),
}

fn load_data(filename: &str) -> Vec<Instruction> {
    let data = fs::read_to_string(filename).unwrap();
    let mut instructions = Vec::new();
    let re = Regex::new(r"(turn on|turn off|toggle)\s+(\d+),(\d+) through (\d+),(\d+).*").unwrap();
    for line in data.lines() {
        for cap in re.captures_iter(line) {
            let action = match &cap[1] {
                "turn on" => Action::TurnOn,
                "turn off" => Action::TurnOff,
                "toggle" => Action::Toggle,
                _ => panic!("Unknown action: {}", &cap[1]),
            };
            let start = (usize::from_str(&cap[2]).unwrap(), usize::from_str(&cap[3]).unwrap());
            let end = (usize::from_str(&cap[4]).unwrap(), usize::from_str(&cap[5]).unwrap());
            instructions.push(Instruction { action, start, end });
        }
    }
    instructions
}

#[named]
fn part1() {
    let instructions = load_data(&format!("src\\d{}\\data.txt", DAY));
    let mut board = alloc_2d_vec(1000, 1000, false);
    for instruction in instructions {
        for x in instruction.start.0..=instruction.end.0 {
            for y in instruction.start.1..=instruction.end.1 {
                match instruction.action {
                    Action::TurnOn => board[x][y] = true,
                    Action::TurnOff => board[x][y] = false,
                    Action::Toggle => board[x][y] = !board[x][y],
                }
            }
        }
    }
    let count = board.iter().map(|row| row.iter().filter(|&b| *b).count()).sum::<usize>();
    println!("{}: {}", function_name!(), count);
}

#[named]
fn part2() {
    let instructions = load_data(&format!("src\\d{}\\data.txt", DAY));
    let mut board = alloc_2d_vec(1000, 1000, 0);
    for instruction in instructions {
        for x in instruction.start.0..=instruction.end.0 {
            for y in instruction.start.1..=instruction.end.1 {
                match instruction.action {
                    Action::TurnOn => board[x][y] += 1,
                    Action::TurnOff => board[x][y] = cmp::max(0, board[x][y] - 1),
                    Action::Toggle => board[x][y] += 2,
                }
            }
        }
    }
    let total_brightness = board.iter().map(|row| row.iter().sum::<i32>()).sum::<i32>();
    println!("{}: {}", function_name!(), total_brightness);
}

pub fn run() {
    println!("Day {}:", DAY);
    part1();
    part2();
}

// part1: 377891
// part2: 14110788
 