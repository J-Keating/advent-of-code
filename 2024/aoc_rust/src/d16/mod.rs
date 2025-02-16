use function_name::named;
//use itertools::Itertools;
use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}, fmt};
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct Progress {
    loc: PointRC,
    direction: Direction,
    cost: usize,
    dupe_count: usize,
    path: HashSet<PointRC>,
}

impl Progress {
    fn can_combine(&self, other: &Self) -> bool {
        self.loc == other.loc && self.direction == other.direction && self.cost == other.cost
    }

    fn combine(&self, other: &Self) -> Progress {
        assert!(self.can_combine(other));
        Progress {
            loc: self.loc,
            direction: self.direction,
            cost: self.cost,
            dupe_count: self.dupe_count + other.dupe_count,
            path: self.path.union(&other.path).copied().collect(),
        }
    }
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

    let board = board_in.remap(|_, c| match c {
        '#' => MapState::Blocked,
        '.' | 'S' | 'E' => MapState::Open,
        _ => panic!("Unexpected char"),
    });

    (board, start, end)
}

#[allow(dead_code)]
fn print_with_progress(board: &Board<MapState>, progress: &BinaryHeap<Progress>) {
    let mut board_display = board.remap(|_, map_state| match map_state {
        MapState::Blocked => '#',
        MapState::Open => '.',
        MapState::Visited => '0',
    });
    for p in progress {
        board_display[p.loc] = p.direction.to_char();
    }
    board_display.print();
}

fn shortest_path(board: &mut Board<MapState>, start: PointRC, end: PointRC) -> Option<(usize, usize)> {
    let mut queue = BinaryHeap::new();
    queue.push(Progress {
        loc: start,
        direction: Direction::Right,
        cost: 0,
        dupe_count: 1,
        path: HashSet::new(),
    });

    while let Some(Progress { loc, direction, cost, dupe_count, mut path }) = queue.pop() {
        if loc == end {
            return Some((cost, path.len() + 1));
        }

        board[loc] = MapState::Visited;
        path.insert(loc);
        let front = loc.step_in_direction(direction);
        let front_progress = Progress { loc: front, direction, cost: cost + 1, dupe_count, path: path.clone() };
        if board[front] == MapState::Open {
            queue.push(front_progress);
        }
        else if queue.iter().find(|p| p.can_combine(&front_progress)).is_some() {
            let mut new_queue = BinaryHeap::new();
            for p in queue.drain() {
                new_queue.push(if p.can_combine(&front_progress) { p.combine(&front_progress) } else { p } );
            }
            queue = new_queue;
        }
        let right_dir = direction.turn_right();
        if board[loc.step_in_direction(right_dir)] == MapState::Open {
            queue.push(Progress { loc, direction: right_dir, cost: cost + 1000, dupe_count, path: path.clone() });
        }
        let left_dir = direction.turn_left();
        if board[loc.step_in_direction(left_dir)] == MapState::Open {
            queue.push(Progress { loc, direction: left_dir, cost: cost + 1000, dupe_count, path: path.clone() });
        }
        // print_with_progress(board, &queue);
        // println!("{:?}", queue);
        // println!();
    }

    None
}

#[named]
fn part1() {
    use real_data as data;
    let (mut board, start, end) = load_data(data::FILENAME);
    let (cost, _) = shortest_path(&mut board, start, end).unwrap();
    println!("{}: {}", function_name!(), cost);
}

#[named]
fn part2() {
    use real_data as data;
    let (mut board, start, end) = load_data(data::FILENAME);
    let (_, tile_count) = shortest_path(&mut board, start, end).unwrap();
    println!("{}: {}", function_name!(), tile_count);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 93436
// part2: 486
