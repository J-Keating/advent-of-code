// use itertools::Itertools;
use std::{collections::HashMap};
use ::function_name::named;
use utilities::{Board, PointRC};

const DAY: &str = "d10";

fn find_summits_from(board: &Board<i32>, loc: &PointRC, summits: &mut HashMap<PointRC, usize>) {
    let curr_height = board.at(loc);
    if curr_height == &9 {
        summits.entry(*loc).and_modify(|e| *e += 1).or_insert(1);
        return;
    }
    for n in loc.neighbors_cardinal() {
        if board.in_bounds(&n) && board.at(&n) - curr_height == 1 {
            find_summits_from(&board, &n, summits);
        }
    }
}

fn count_summits_from(board: &Board<i32>, loc: &PointRC) -> usize {
    let mut summits = HashMap::new();
    find_summits_from(board, loc, &mut summits);
    summits.len()
}

fn count_paths_to_summits_from(board: &Board<i32>, loc: &PointRC) -> usize {
    let mut summits = HashMap::new();
    find_summits_from(board, loc, &mut summits);
    summits.iter().map(|(_, v)| v).sum()
}

#[named]
fn part1() {
    let board = Board::<i32>::load_data_int(&("src\\".to_string() + DAY + "\\data.txt"));
    let starts = board.find_all(0);
    let summit_count = starts.iter().map(|p| count_summits_from(&board, &p)).sum::<usize>();
    println!("{}: {:?}", function_name!(), summit_count);
}

#[named]
fn part2() {
    let board = Board::<i32>::load_data_int(&("src\\".to_string() + DAY + "\\data.txt"));
    let starts = board.find_all(0);
    let path_count = starts.iter().map(|p| count_paths_to_summits_from(&board, &p)).sum::<usize>();
    println!("{}: {:?}", function_name!(), path_count);
}

pub fn run() {
    part1();
    part2();
}

// part1: 798
// part2: 1816
