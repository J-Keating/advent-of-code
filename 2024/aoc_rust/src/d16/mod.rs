use function_name::named;
//use itertools::Itertools;
use std::{cmp::Ordering, collections::BinaryHeap, fmt};
use utilities::{Board, Direction, PointRC};

const DAY: &str = "d16";

#[allow(dead_code)]
mod test_data {
    pub const FILENAME: &str = r"src\d16\data_test2.txt";
}
#[allow(dead_code)]
mod real_data {
    pub const FILENAME: &str = r"src\d16\data.txt";
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

#[derive(Clone, Copy, Eq, PartialEq)]
struct Progress {
    loc: PointRC,
    direction: Direction,
    cost: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Progress {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.loc.r.cmp(&other.loc.r))
            .then_with(|| self.loc.c.cmp(&other.loc.c))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Progress {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn load_data(path: &str) -> (Board<MapState>, PointRC, PointRC) {
    let board_in = Board::<char>::load_data_chars_from_file(path);
    let start = board_in.find_first('S').unwrap();
    let end = board_in.find_first('E').unwrap();

    let board = board_in.remap(|c| match c {
        '#' => MapState::Blocked,
        '.' | 'S' | 'E' => MapState::Open,
        _ => panic!("Unexpected char"),
    });

    (board, start, end)
}

fn shortest_path(board: &mut Board<MapState>, start: PointRC, end: PointRC) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    queue.push(Progress {
        loc: start,
        direction: Direction::Right,
        cost: 0,
    });

    while let Some(Progress { loc, direction, cost }) = queue.pop() {
        if loc == end {
            return Some(cost);
        }

        board[loc] = MapState::Visited;
        let front = loc.step_in_direction(direction);
        if board[front] == MapState::Open {
            queue.push(Progress {
                loc: front,
                direction,
                cost: cost + 1,
            });
        }
        let right_dir = direction.turn_right();
        if board[loc.step_in_direction(right_dir)] == MapState::Open {
            queue.push(Progress {
                loc,
                direction: right_dir,
                cost: cost + 1000,
            });
        }
        let left_dir = direction.turn_left();
        if board[loc.step_in_direction(left_dir)] == MapState::Open {
            queue.push(Progress {
                loc,
                direction: left_dir,
                cost: cost + 1000,
            });
        }
    }

    None
}

#[named]
fn part1() {
    use real_data as data;
    let (mut board, start, end) = load_data(data::FILENAME);
    let result = shortest_path(&mut board, start, end).unwrap();
    println!("{}: {}", function_name!(), result);
}

#[named]
fn part2() {
    use test_data as data;
    let (mut board, start, end) = load_data(data::FILENAME);
    println!("{}: {}", function_name!(), 1);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 1375
// part2: 983054
