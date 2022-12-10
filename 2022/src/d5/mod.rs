use std::{fs};

use ::function_name::named;
use ::itertools::Itertools;

fn print_stacks(stacks: &Vec<Vec<char>>) {
    let debug: bool = false;
    if debug {
        for stack in stacks {
            //cloned()
            println!("{}|", stack.iter().collect::<String>());
        }
        println!("============")
    }
}

#[named]
fn part1() {
    let file_contents = fs::read_to_string("src\\d5\\data.txt").expect("Error loading file");
    if let Some((stacks_line, moves_line)) = file_contents.split("\r\n\r\n").map(|f| f.to_string()).collect_tuple() {

        // Load the lines for the crate stacks from bottom to top
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let stacks_lines = stacks_line.rsplit("\r\n").collect::<Vec<&str>>();

        // Peel off the now-top line and sanity check that it is the stack numbers (eg: " 1   2   3 ")
        let (label_line, stacks_lines) = stacks_lines.split_first().unwrap();
        assert!(label_line.len() >= 3 && label_line.chars().nth(1).unwrap() == '1' && (label_line.len() + 1) % 4 == 0);

        // Allocate vectors for each stack
        stacks.resize((label_line.len() + 1) / 4, Vec::new());

        // Read crate names, which are character positions 1,5,9..., handling the ' ' to mean no crate
        for row in stacks_lines.iter() {
            for (write_index, crate_name) in row.chars().skip(1).step_by(4).enumerate().filter(|(_,name)| name != &' ') {
                stacks[write_index].push(crate_name);
            }
        }
        print_stacks(&stacks);

        // Do the moves
        for single_move in moves_line.split("\r\n") {
            // eg: "move 13 from 6 to 1"
            let tokens = single_move.split(" ").collect::<Vec<&str>>();
            assert!(tokens.len() == 6);
            if let (Ok(count), Ok(from), Ok(to)) = (tokens[1].parse::<usize>(), tokens[3].parse::<usize>(), tokens[5].parse::<usize>()) {
                for _ in 0..count {
                    let crate_name = stacks[from-1].pop().unwrap();
                    stacks[to-1].push(crate_name);
                }
                print_stacks(&stacks);
            }
            else {
                panic!();
            }
        }

        // Build String from characters at the top of each stack
        let res = stacks.iter().fold(String::new(), |res, stack| res + &stack[stack.len() - 1].to_string());
        println!("{}: {}", function_name!(), res);
    }
}

#[named]
fn part2() {
    let file_contents = fs::read_to_string("src\\d5\\data.txt").expect("Error loading file");
    if let Some((stacks_line, moves_line)) = file_contents.split("\r\n\r\n").map(|f| f.to_string()).collect_tuple() {
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let stacks_lines = stacks_line.rsplit("\r\n").collect::<Vec<&str>>();
        assert!((stacks_lines[0].len() + 1) % 4 == 0);
        stacks.resize((stacks_lines[0].len() + 1) / 4, Vec::new());
        for row in stacks_lines.iter().skip(1) {
            let buff = row.as_bytes();
            let mut read_index = 1;
            let mut write_index = 0;
            while read_index < buff.len() {
                let crate_name = buff[read_index] as char;
                if crate_name != ' ' {
                    stacks[write_index].push(crate_name);
                }
                write_index += 1;
                read_index += 4;
            }
        }
        print_stacks(&stacks);
        for single_move in moves_line.split("\r\n") {
            let tokens = single_move.split(" ").collect::<Vec<&str>>();
            assert!(tokens.len() == 6);
            let (count, from, to) = (tokens[1].parse::<usize>().unwrap(), tokens[3].parse::<usize>().unwrap(), tokens[5].parse::<usize>().unwrap());
            let mut scratch: Vec<char> = Vec::new();
            for _ in 0..count {
                let crate_name = stacks[from-1].pop().unwrap();
                scratch.push(crate_name);
            }
            for &c in scratch.iter().rev() {
                stacks[to-1].push(c);
            }
            print_stacks(&stacks);
        }
        let mut res: String = "".to_owned();
        for stack in &stacks {
            let next = stack[stack.len() - 1].to_string();
            res.push_str(&next);
        }
        println!("{}: {}", function_name!(), res);
    }
}

pub fn run() {
    part1();
    part2();
}

// part1: QPJPLMNNR
// part2: BQDNWJPVJ