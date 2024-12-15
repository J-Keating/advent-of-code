// use itertools::Itertools;
use std::{collections::HashSet, fs};
// use std::collections::{HashMap, HashSet};
use ::function_name::named;
use utilities::{Board, PointRC};

const DAY: &str = "d10";

fn find_summits_from(board: &Board<i32>, loc: &PointRC, summits: &mut HashSet<PointRC>) {
    let curr_height = board.at(loc);
    if curr_height == &9 {
        summits.insert(*loc);
        return;
    }
    for n in loc.neighbors_cardinal() {
        if board.in_bounds(&n) && board.at(&n) - curr_height == 1 {
            find_summits_from(&board, &n, summits);
        }
    }
}

fn count_summits_from(board: &Board<i32>, loc: &PointRC) -> usize {
    let mut summits = HashSet::new();
    find_summits_from(board, loc, &mut summits);
    summits.len()
}

#[named]
fn part1() {
    let board = Board::<i32>::load_data_int(&("src\\".to_string() + DAY + "\\data.txt"));
    let starts = board.find_all(0);
    let path_count = starts.iter().map(|p| count_summits_from(&board, &p)).sum::<usize>();
    println!("{}: {:?}", function_name!(), path_count);
}

#[named]
fn part2() {
    let file_contents_as_string =
        fs::read_to_string(&("src\\".to_string() + DAY + "\\data_test.txt"))
            .expect("Error loading file");
    println!("{}: {}", function_name!(), file_contents_as_string.len());
}

pub fn run() {
    part1();
    part2();
}

// part1: 798
// part2: 
