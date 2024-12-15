use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use utilities::{Board, PointRC};
use ::function_name::named;

const DAY: &str = "d8";

fn find_char_locations(board: &Board<char>) -> HashMap<char, Vec<PointRC>> {
    let mut ret = HashMap::new();
    for r in 0..board.height {
        for c in 0..board.width {
            let ch = board.data[r][c];
            if ch != '.' {
                if !ret.contains_key(&ch) {
                    ret.insert(ch, Vec::new());
                }
                ret.get_mut(&ch).unwrap().push(PointRC { r: r as i32, c: c as i32 });
            }
        }
    }
    ret
}

#[named]
fn part1() {
    let board = Board::<char>::load_data_chars(&("src\\".to_string() + DAY + "\\data.txt"));
    let locs = find_char_locations(&board);
    let mut antinode_locs = HashSet::<PointRC>::new();
    for (_, locs) in locs.iter() {
        for (a, b) in locs.iter().tuple_combinations() {
            let diff = a.sub(b);
            // diff = a - b, so a = b + diff
            antinode_locs.insert(a.add(&diff));
            antinode_locs.insert(b.sub(&diff));
        }
    }
    antinode_locs = antinode_locs.into_iter().filter(|p| board.in_bounds(p)).collect();
    println!("{}: {}", function_name!(), antinode_locs.len());
}

#[named]
fn part2() {
    let board = Board::<char>::load_data_chars(&("src\\".to_string() + DAY + "\\data.txt"));
    let locs = find_char_locations(&board);
    let mut antinode_locs = HashSet::<PointRC>::new();
    for (_, locs) in locs.iter() {
        for (a, b) in locs.iter().tuple_combinations() {
            let mut diff = a.sub(b);
            diff = diff.div(num::integer::gcd(diff.r, diff.c));
            let mut loc = a.clone();
            while board.in_bounds(&loc) {
                antinode_locs.insert(loc);
                loc = loc.add(&diff);
            }
            loc = a.sub(&diff);
            while board.in_bounds(&loc) {
                antinode_locs.insert(loc);
                loc = loc.sub(&diff);
            }
        }
    }
    antinode_locs = antinode_locs.into_iter().filter(|p| board.in_bounds(p)).collect();
    println!("{}: {}", function_name!(), antinode_locs.len());
}

pub fn run() {
    part1();
    part2();
}

// part1: 327
// part2: 1233
