use function_name::named;
//use itertools::Itertools;
use std::fmt;
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

fn load_data(path: &str) -> (Board<MapState>, PointRC, PointRC) {
    let board_in = Board::<char>::load_data_chars_from_file(path);
    let start = board_in.find_first('S').unwrap();
    let end = board_in.find_first('E').unwrap();

    let board = board_in.remap(|_, c| match c {
        '#' => MapState::Blocked,
        '.' | 'S' | 'E' => MapState::Open,
        _ => panic!("Unexpected char"),
    });

    (board, start, end)
}

fn find_path(board: &mut Board<MapState>, start: PointRC, end: PointRC) -> Vec<PointRC> {
    let mut path = Vec::<PointRC>::new();
    let mut current = start;
    while current != end {
        board[current] = MapState::Visited;
        path.push(current);
        let all_possible = current
            .neighbors_cardinal()
            .iter()
            .filter(|&p| board.in_bounds(p) && board[*p] == MapState::Open)
            .map(|&p| p.clone())
            .collect::<Vec<PointRC>>();
        assert!(all_possible.len() == 1);
        current = all_possible[0];
    }
    path.push(end);
    path
}

fn count_successful_cheats(path: &[PointRC], max_cheat: usize, min_savings: usize) -> usize {
    //let min_savings = 100;
    let mut count = 0;
    for i in 0..(path.len() - min_savings) {
        for j in (i + min_savings + 1)..path.len() {
            let path_cost = j - i;
            let cheat_cost = path[i].manhattan_dist(&path[j]) as usize;
            if cheat_cost <= max_cheat && (path_cost - cheat_cost) >= min_savings {
                count += 1;
            }
        }
    }
    count
}

#[named]
fn part1() {
    use real_data as data;
    let (mut board, start, end) = load_data(data::FILENAME);
    let path = find_path(&mut board, start, end);
    let count = count_successful_cheats(&path, 2, 100);
    println!("{}: {}", function_name!(), count);
}

#[named]
fn part2() {
    use real_data as data;
    let (mut board, start, end) = load_data(data::FILENAME);
    let path = find_path(&mut board, start, end);
    let count = count_successful_cheats(&path, 20, 100);
    println!("{}: {}", function_name!(), count);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 1375
// part2: 983054
