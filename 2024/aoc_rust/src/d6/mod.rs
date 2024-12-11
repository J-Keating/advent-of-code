use std::collections::{HashSet};
use std::{fs};

use ::function_name::named;
use utilities::alloc_2d_vec;
use utilities::PointRC;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Empty
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
        _ => panic!("Invalid direction")
    }
}

fn step(direction: Direction) -> PointRC {
    match direction {
        Direction::Up => PointRC { r: -1, c: 0 },
        Direction::Down => PointRC { r: 1, c: 0 },
        Direction::Left => PointRC { r: 0, c: -1 },
        Direction::Right => PointRC { r: 0, c: 1 },
        _ => panic!("Invalid direction"),
    }
}

struct Board {
    pub data: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
}

impl Board {
    pub fn new(height: usize, width: usize) -> Board {
        Board {
            data: alloc_2d_vec::<char>(height, width, ' '),
            width,
            height,
        }
    }

    #[allow(dead_code)]
    pub fn print(&self, path: &HashSet<PointRC>) {
        let mut buff = self.data.clone();
        for point in path {
            buff[point.r as usize][point.c as usize] = 'x';
        }
        for line in buff {
            println!("{}", line.iter().collect::<String>());
        }
    }

    #[allow(dead_code)]
    pub fn print2(&self, visited: &Vec<Vec<Direction>>) {
        let mut buff = self.data.clone();
        for r in 0..self.height {
            for c in 0..self.width {
                buff[r][c] = match visited[r][c] {
                    Direction::Up => '^',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                    Direction::Right => '>',
                    Direction::Empty => ' ',
                };
            }
        }
        for line in buff {
            println!("{}", line.iter().collect::<String>());
        }
    }

    pub fn find(&self, target: char) -> Option<PointRC> {
        for r in 0..self.height {
            for c in 0..self.width {
                if self.data[r][c] == target {
                    return Some(PointRC { r: r as i32, c: c as i32 });
                }
            }
        }
        None
    }

    pub fn in_bounds(&self, loc: PointRC) -> bool {
        loc.r >= 0 && loc.r < self.height as i32 && loc.c >= 0 && loc.c < self.width as i32
    }
}

fn load_data(path: &str) -> Board {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let file_lines = file_contents_as_string.lines().collect::<Vec<&str>>();
    let height = file_lines.len();
    let width = file_lines.iter().map(|x| x.len()).max().unwrap();
    let mut board = Board::new(height, width);
    for (row, line) in file_lines.iter().enumerate() {
        for (col, state) in line.chars().enumerate() {
            board.data[row][col] = state;
        }
    }
    board
}

fn run_simulation(board: &Board, start_loc: PointRC) -> HashSet<PointRC> {
    let mut loc = start_loc;
    let mut dir = Direction::Up;
    let mut visited = HashSet::new();
    visited.insert(loc);
    loop {
        let next_loc = loc.add(&step(dir));
        if !board.in_bounds(next_loc) {
            break;
        }
        if board.data[next_loc.r as usize][next_loc.c as usize] == '#' {
            dir = turn_right(dir);
        }
        else {
            loc = next_loc;
            visited.insert(loc);
        }
    }
    visited
}
#[named]
fn part1() {
    let board = load_data("src\\d6\\data.txt");
    let loc = board.find('^').unwrap();
    let visited = run_simulation(&board, loc);
    println!("{}: {}", function_name!(), visited.len());
}

fn does_loop(board: &Board, start_loc: PointRC, start_dir: Direction) -> bool {
    let mut loc = start_loc;
    let mut dir = start_dir;
    let mut visited = alloc_2d_vec::<Direction>(board.height, board.width, Direction::Empty);

    let mut count = 0;
    loop {
        visited[loc.r as usize][loc.c as usize] = dir;
        let next_loc = loc.add(&step(dir));
        if !board.in_bounds(next_loc) {
            return false;
        }
        if board.data[next_loc.r as usize][next_loc.c as usize] == '#' {
            dir = turn_right(dir);
        }
        else {
            loc = next_loc;
        }
        if visited[loc.r as usize][loc.c as usize] == dir {
            return true;
        }
        count += 1;
        if count > 1 * 1000 * 1000 {
            //board.print2(&visited);
            return true;
        }
    }
}

#[named]
fn part2() {
    let mut board = load_data("src\\d6\\data.txt");
    let start_loc = board.find('^').unwrap();
    let mut visited = run_simulation(&board, start_loc);
    visited.remove(&start_loc);

    let mut block_locs: HashSet<PointRC> = HashSet::new();
    for test_loc in visited.iter() {
        let old = board.data[test_loc.r as usize][test_loc.c as usize];
        board.data[test_loc.r as usize][test_loc.c as usize] = '#';
        if does_loop(&board, start_loc, Direction::Up) {
            block_locs.insert(*test_loc);
        }
        board.data[test_loc.r as usize][test_loc.c as usize] = old;
    }
    println!("{}: {}", function_name!(), block_locs.len());
}

pub fn run() {
    part1();
    part2();
}

// part1: 4752
// part2: 1719