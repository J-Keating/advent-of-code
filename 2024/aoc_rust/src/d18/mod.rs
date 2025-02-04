use function_name::named;
//use itertools::Itertools;
use std::{collections::VecDeque, fmt, fs};
use utilities::{Board, PointRC};

const DAY: &str = "d18";

#[allow(dead_code)]
mod test_data {
    pub const FILENAME: &str = r"src\d18\data_test.txt";
    pub const DIMS: usize = 7;
    pub const dropped: usize = 12;
}
#[allow(dead_code)]
mod real_data {
    pub const FILENAME: &str = r"src\d18\data.txt";
    pub const DIMS: usize = 71;
    pub const dropped: usize = 1024;
}

#[derive(Clone, Copy, PartialEq)]
enum TileState {
    Open,
    Blocked,
    Dist(i32),    
}

impl fmt::Display for TileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TileState::Open => write!(f, "."),
            TileState::Blocked => write!(f, "#"),
            TileState::Dist(d) => write!(f, "{}", d % 10),
        }
    }
}

fn load_data(path: &str) -> Vec<PointRC> {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    file_contents_as_string
        .lines()
        .map(|s| {
            let mut parts = s.split(",");
            let x = parts.next().unwrap().parse::<i32>().unwrap();
            let y = parts.next().unwrap().parse::<i32>().unwrap();
            PointRC { r: y, c: x }
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PointToVisit {
    loc: PointRC,
    dist: i32,
}

fn solve_board(board: &mut Board<TileState>, start_loc: PointRC, end_loc: PointRC) -> Option<i32> {
    let mut positions: VecDeque<PointToVisit> = VecDeque::new();
    positions.push_back( PointToVisit { loc: start_loc, dist: 0 } );

    let mut found_dist = None;
    while let Some(p) = positions.pop_front() {
        let take_location = board.in_bounds(&p.loc) && match board[p.loc] {
            TileState::Open => true,
            TileState::Blocked => false,
            TileState::Dist(d) => p.dist < d,
        };
        if p.loc == end_loc {
            //board.print();
            //println!("Found!: {} {}", p.dist, p.loc);
            found_dist = Some(p.dist);
            break;
        }
        if take_location {
            board[p.loc] = TileState::Dist(p.dist);
            p.loc.neighbors_cardinal().iter().for_each(|&n| {
                positions.push_back( PointToVisit { loc: n, dist: p.dist + 1 } );
            });
        }
    }
    found_dist
}

#[named]
fn part1() {
    use real_data as data;
    let points = load_data(data::FILENAME);
    let start_loc = PointRC { r: 0, c: 0 };
    let end_loc = PointRC { r: data::DIMS as i32 - 1, c: data::DIMS as i32 - 1 };

    let mut board = Board::<TileState>::new(data::DIMS, data::DIMS, TileState::Open);
    points[0..data::dropped].iter().for_each(|&f| board[f] = TileState::Blocked);

    let found_dist = solve_board(&mut board, start_loc, end_loc);

    println!("{}: {}", function_name!(), if found_dist.is_some() { found_dist.unwrap() } else { -1 });
}

#[named]
fn part2() {
    use test_data as data;
    let _ = load_data(data::FILENAME);
    println!("{}: {}", function_name!(), data::DIMS);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 2,1,0,1,7,2,5,0,3
// Found!: 267265166222235 (0o7461160522621633): 2,4,1,7,7,5,0,3,4,4,1,7,5,5,3,0
