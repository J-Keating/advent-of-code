use function_name::named;
//use itertools::Itertools;
use std::{collections::VecDeque, fmt};
use utilities::{Board, PointRC};

const DAY: &str = "d20";

#[allow(dead_code)]
mod test_data {
    pub const FILENAME: &str = r"src\d20\data_test.txt";
}
#[allow(dead_code)]
mod real_data {
    pub const FILENAME: &str = r"src\d20\data.txt";
}

#[derive(Clone, Copy, PartialEq)]
enum MapState {
    Blocked,
    Open,
    Visited,
}

impl Default for MapState {
    fn default() -> Self {
        MapState::Open
    }
}

impl fmt::Display for MapState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MapState::Blocked => write!(f, "#"),
            MapState::Open => write!(f, "."),
            MapState::Visited => write!(f, "0"),
        }
    }
}

fn load_data(path: &str) -> (Board<char>, PointRC, PointRC) {
    let board = Board::<char>::load_data_chars_from_file(path);
    let start = board.find_first('S').unwrap();
    let end = board.find_first('E').unwrap();
    (board, start, end)
}

fn step(board: &mut Board<MapState>, current: PointRC, end: &PointRC, path: &mut Vec<PointRC>) -> bool {
    board[current] = MapState::Visited;
    path.push(current);
    if current == *end {
        return true;
    }
    for next in current.neighbors_cardinal() {
        if board.in_bounds(&next) && board[next] == MapState::Open {
            if step(board, next, end, path) {
                return true;
            }
        }
    }
    path.pop();
    false
}

#[named]
fn part1() {
    use real_data as data;
    let (board_in, start, end) = load_data(data::FILENAME);
    let mut board = board_in.remap(|c| match c {
        '#' => MapState::Blocked,
        '.' | 'S' | 'E' => MapState::Open,
        _ => panic!("Unexpected char"),
    });
    let mut path = Vec::<PointRC>::new();
    let mut to_process = VecDeque::<PointRC>::new();
    to_process.push_back(start);
    while let Some(current) = to_process.pop_front() {
        board[current] = MapState::Visited;
        path.push(current);
        if current == end {
            break;
        }
        for next in current.neighbors_cardinal() {
            if board.in_bounds(&next) && board[next] == MapState::Open {
                to_process.push_back(next);
            }
        }
    }
    // let mut path2 = Vec::<PointRC>::new();
    // let success = step(&mut board, start, &end, &mut path2);
    // assert!(success && path == path2);

    let is_cheat = |i: usize, j: usize| { 
        let p: &PointRC = &path[i];
        let q: &PointRC = &path[j];
        (p.r == q.r || p.c == q.c) && (*p).manhattan_dist(&q) == 2 && board[p.add(&q).div(2)] == MapState::Blocked
    };
    
    /* Test Data
    for diff in 3..=70 {
        let mut count = 0;
        for i in 0..(path.len() - diff) {
            let j = i + diff;
            if is_cheat(i, j) {
                //println!("{}: {}=>{}", diff - 2, path[i], path[j]);
                count += 1;
            }
        }
        if count > 0 {
            println!("{}: {}", diff - 2, count);
        }
    }
    */
    
    let savings = 100;
    let mut count = 0;
    for i in 0..(path.len() - savings) {
        for j in (i + savings + 2)..path.len() {
            if is_cheat(i, j) {
                //println!("{}: {}=>{}", diff - 2, path[i], path[j]);
                count += 1;
            }
        }
    }

    println!("{}: {}", function_name!(), count);
}

#[named]
fn part2() {
    use test_data as data;
    let _ = load_data(data::FILENAME);
    println!("{}: {}", function_name!(), 0);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 1375
// part2: 0