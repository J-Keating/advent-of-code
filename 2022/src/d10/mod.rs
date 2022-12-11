use std::{fs};

use ::function_name::named;
use itertools::Itertools;
//use lazy_static::lazy_static;

#[named]
fn part1() {
    let file_contents = fs::read_to_string("src\\d10\\data.txt").expect("Error loading file");
    let mut file_lines = file_contents.split("\r\n");

    let mut iteration_count = 0;
    let mut register = 1;
    let mut pending_write: Option<i32> = None;
    let mut total_strength = 0;

    loop {
        iteration_count += 1;

        // Use Register here
        //println!("{}: {} ({:?})", iteration_count, register, pending_write);
        if (iteration_count - 20) % 40 == 0 {
            total_strength += iteration_count * register;
        }

        if pending_write.is_some() {
            register = pending_write.unwrap();
            pending_write = None;
            continue;
        }

        let next_line = file_lines.next();
        if next_line.is_none() {
            break;
        }
        
        let instruction = next_line.unwrap().split(' ').collect::<Vec<&str>>();
        assert!((instruction.len() == 1 && instruction[0] == "noop") || (instruction.len() == 2 && instruction[0] == "addx"));
        match instruction[0] {
            "noop" => { }
            "addx" => {
                pending_write = Some(register + instruction[1].parse::<i32>().unwrap());
            }
            _ => { panic!() }
        }
    }
    println!("{}: {}", function_name!(), total_strength);
}

#[named]
fn part2() {
    let file_contents = fs::read_to_string("src\\d10\\data.txt").expect("Error loading file");
    let mut file_lines = file_contents.split("\r\n");

    let mut iteration_count = 0;
    let mut register = 1;
    let mut pending_write: Option<i32> = None;
    
    println!("{}:", function_name!());
    loop {
        iteration_count += 1;
        let crt_index = (iteration_count - 1) % 40;

        print!("{}", if num::abs(crt_index - register) < 2 { '#' } else { '.'});

        if iteration_count % 40 == 0 {
            println!("");
        }

        if pending_write.is_some() {
            register = pending_write.unwrap();
            pending_write = None;
            continue;
        }

        let next_line = file_lines.next();
        if next_line.is_none() {
            break;
        }

        let instruction = next_line.unwrap().split(' ').collect::<Vec<&str>>();
        assert!((instruction.len() == 1 && instruction[0] == "noop") || (instruction.len() == 2 && instruction[0] == "addx"));
        match instruction[0] {
            "noop" => { }
            "addx" => {
                pending_write = Some(register + instruction[1].parse::<i32>().unwrap());
            }
            _ => { panic!() }
        }
    }
}

pub fn run() {
    part1();
    part2();
}

// part1: 11220
// part2:
// ###..####.###...##....##.####.#....#..#.
// #..#....#.#..#.#..#....#.#....#....#.#..
// ###....#..#..#.#..#....#.###..#....##...
// #..#..#...###..####....#.#....#....#.#..
// #..#.#....#....#..#.#..#.#....#....#.#..
// ###..####.#....#..#..##..####.####.#..#.