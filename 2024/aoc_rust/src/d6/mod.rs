use std::collections::HashSet;
use std::{fs};

use ::function_name::named;
use utilities::alloc_2d_vec;
use utilities::PointRC;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down
    }
}

fn step(direction: Direction) -> PointRC {
    match direction {
        Direction::Up => PointRC { r: -1, c: 0 },
        Direction::Down => PointRC { r: 1, c: 0 },
        Direction::Left => PointRC { r: 0, c: -1 },
        Direction::Right => PointRC { r: 0, c: 1 }
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

#[named]
fn part1() {
    let mut board = load_data("src\\d6\\data.txt");
    let mut loc = board.find('^').unwrap();
    board.data[loc.r as usize][loc.c as usize] = '.';
    let mut dir = Direction::Up;
    let mut visited: HashSet<PointRC> = HashSet::new();
    loop {
        visited.insert(loc);
        let next_loc = loc.add(&step(dir));
        if !board.in_bounds(next_loc) {
            break;
        }
        if board.data[next_loc.r as usize][next_loc.c as usize] != '.' {
            dir = turn_right(dir);
        }
        else {
            loc = next_loc;
        }
        //board.print(&visited);
    }
    board.print(&visited);
    println!("{}: {}", function_name!(), visited.len());
}

#[named]
fn part2() {
    let board = load_data("src\\d5\\data_test.txt");
    println!("{}: {}", function_name!(), board.height);

}

pub fn run() {
    part1();
    part2();
}

// part1: 5509
// part2: 4407
