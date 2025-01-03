//use std::{fs, fmt};

use std::collections::HashMap;

use ::function_name::named;
//use itertools::Itertools;
//use itertools::Itertools;
//use lazy_static::lazy_static;
use utilities::{Board, PointRC};

const DAY: &str = "d14";

#[derive(Debug)]
struct Plot {
    pub area: i32,
    pub permimeter: i32
}

fn count_permimiter(board: &Board<char>, row: i32, col: i32) -> i32 {
    let plot = board.data[row as usize][col as usize];
    let locs = vec![PointRC { r: row, c: col - 1 }, PointRC { r: row, c: col + 1 }, PointRC { r: row - 1, c: col }, PointRC { r: row + 1, c: col }];
    locs.iter().filter(|loc| !board.in_bounds(loc) || board.data[loc.r as usize][loc.c as usize] != plot).count() as i32
}

fn do_part_1(filename: &str) {
    let board = Board::<char>::load_data_chars(filename);
    let mut plot_info = HashMap::<char, Plot>::new();
    for r in 0..board.height {
        for c in 0..board.width {
            let plot = board.data[r][c];
            let plot_info = plot_info.entry(plot).or_insert(Plot { area: 0, permimeter: 0 });
            plot_info.area += 1;
            plot_info.permimeter += count_permimiter(&board, r as i32, c as i32);
        }
    }
    let total_cost = plot_info.iter().fold(0, |acc, (_, plot)| acc + plot.area * plot.permimeter);
    println!("{:?}", plot_info);
    println!("part1: {}: {}", filename, total_cost);
}

#[named]
fn part1() {
    // let board = Board::<char>::load_data_chars(&("src\\".to_string() + DAY + "\\data_test_2.txt"));
    // let mut plot_info = HashMap::<char, Plot>::new();
    // for r in 0..board.height {
    //     for c in 0..board.width {
    //         let plot = board.data[r][c];
    //         let plot_info = plot_info.entry(plot).or_insert(Plot { area: 0, permimeter: 0 });
    //         plot_info.area += 1;
    //         plot_info.permimeter += count_permimiter(&board, r as i32, c as i32);
    //     }
    // }
    // let total_cost = plot_info.iter().fold(0, |acc, (_, plot)| acc + plot.area * plot.permimeter);
    // //println!("{:?}", plot_info);
    do_part_1(&("src\\".to_string() + DAY + "\\data_test.txt"));
    do_part_1(&("src\\".to_string() + DAY + "\\data_test_2.txt"));
    do_part_1(&("src\\".to_string() + DAY + "\\data_test_3.txt"));
    // do_part_1(&("src\\".to_string() + DAY + "\\data.txt"));
    println!("{}: {}", function_name!(), "done");
}

#[named]
fn part2() {
    let rested_sand_count = 1;
    println!("{}: {}", function_name!(), rested_sand_count);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 779
// part2: 1