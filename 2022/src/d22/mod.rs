use std::{fs};

use ::function_name::named;
use itertools::{Itertools, iterate};
use utilities::{alloc_2d_vec};
//use lazy_static::lazy_static;

#[test]
fn test_iterator() {
    let x = iterate(1, |x| { println!(".{}", x); x + 1 })
        .filter(|x| x % 3 == 0)
        .take(4)
        .take_while(|x| { println!("    {}", x); x < &20 })
        //.for_each(|x| println!("        {}", x));
        .last().unwrap();
    println!("x={:?}", x);
}

#[allow(dead_code)]
fn increment(x: i32) -> i32 {
    println!("{} ==> {}", x, x + 1);
    x + 1
}

#[test]
fn test_iterator2() {
    let x = iterate(0, |x| increment(*x))
        .skip(1)
        //.filter(|x| x % 3 == 0)
        .take(1)
        //.take_while(|x| { println!("    {}", x); x < &20 })
        //.for_each(|x| println!("        {}", x));
        .last().unwrap();
    println!("x={:?}", x);
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Copy, Debug)]
enum TurnDirection {
    CW,
    CCW,
    AROUND
}

#[derive(Clone, Copy, Debug)]
enum Action {
    Move(usize),
    Turn(TurnDirection),
}

fn turn(direction: Direction, turn_direction: TurnDirection) -> Direction {
    match (direction, turn_direction) {
        (Direction::Up, TurnDirection::CW) => Direction::Right,
        (Direction::Up, TurnDirection::CCW) => Direction::Left,
        (Direction::Up, TurnDirection::AROUND) => Direction::Down,

        (Direction::Down, TurnDirection::CW) => Direction::Left,
        (Direction::Down, TurnDirection::CCW) => Direction::Right,
        (Direction::Down, TurnDirection::AROUND) => Direction::Up,

        (Direction::Left, TurnDirection::CW) => Direction::Up,
        (Direction::Left, TurnDirection::CCW) => Direction::Down,
        (Direction::Left, TurnDirection::AROUND) => Direction::Right,

        (Direction::Right, TurnDirection::CW) => Direction::Down,
        (Direction::Right, TurnDirection::CCW) => Direction::Up,
        (Direction::Right, TurnDirection::AROUND) => Direction::Left,
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
    pub fn print(&self, path: &Vec<(i32, i32, Direction)>) {
        let mut buff = self.data.clone();
        for (r, c, d) in path {
            buff[*r as usize][*c as usize] = match d {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            };
        }
        for line in buff {
            println!("{}", line.iter().collect::<String>());
        }
    }
}

fn load_data(path: &str) -> (Board, Vec<Action>) {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let (grid_string, command_string) = file_contents_as_string.split("\r\n\r\n").collect_tuple().unwrap();
    let file_lines = grid_string.lines().collect::<Vec<&str>>();
    let height = file_lines.len();
    let width = file_lines.iter().map(|x| x.len()).max().unwrap();
    let mut board = Board::new(height, width);
    for (row, line) in file_lines.iter().enumerate() {
        for (col, state) in line.chars().enumerate() {
            board.data[row][col] = state;
        }
    }
    let mut actions: Vec<Action> = Vec::new();
    let commands_with_spaces = command_string.replace("R", " R ").replace("L", " L ");
    for command in commands_with_spaces.split_whitespace() {
        match command {
            "R" => { actions.push(Action::Turn(TurnDirection::CW)); }
            "L" => { actions.push(Action::Turn(TurnDirection::CCW)); }
            " " => {}
            _ => { actions.push(Action::Move(command.parse::<usize>().unwrap())); }
        }
    }
    (board, actions)
}

#[derive(Clone, Copy, PartialEq)]
struct State {
    pub row: i32,
    pub col: i32,
    pub direction: Direction,
}

impl State {
    pub fn rotate(&self, turn_direction: TurnDirection) -> State {
        let mut ret = *self;
        (ret.row, ret.col) = match turn_direction {
            TurnDirection::CW => (self.col, -self.row),
            TurnDirection::CCW => (-self.col, self.row),
            TurnDirection::AROUND => (-self.row, -self.col),
        };
        ret.direction = turn(self.direction, turn_direction);
        ret
    }

    pub fn translate(&self, row: i32, col: i32) -> State {
        let mut ret = *self;
        ret.row += row;
        ret.col += col;
        ret
    }

    pub fn rotate_about(&self, turn_direction: TurnDirection, row: i32, col: i32) -> State {
        let mut ret = *self;
        ret = ret.translate(-row, -col);
        ret = ret.rotate(turn_direction);
        ret = ret.translate(row, col);
        ret
    }
}

fn move_one_with_2d_wrap(mut state: State, board: &Board) -> State {
    match state.direction {
        Direction::Up => {
            state.row = (state.row + board.height as i32 - 1) % board.height as i32;
        }
        Direction::Down => {
            state.row = (state.row + 1) % board.height as i32;
        }
        Direction::Left => {
            state.col = (state.col + board.width  as i32 - 1) % board.width as i32;
        }
        Direction::Right => {
            state.col = (state.col + 1) % board.width as i32;
        }
    };
    // println!("move function return:");
    // board.print(&state);
    state
}

fn move_as_cube_test(mut state: State, board: &Board) -> State {
    match state.direction {
        Direction::Up => {
            match (state.row, state.col) {
                (4, 0..=3) => { state = state.rotate_about(TurnDirection::AROUND, 4, 3).translate(4, 5); }
                (4, 4..=7) => { state = state.rotate_about(TurnDirection::CW, 4, 8); }
                (0, 8..=11) => { state = state.rotate_about(TurnDirection::AROUND, 0, 8).translate(4, -5); }
                (8, 12..=15) => { state = state.rotate_about(TurnDirection::CCW, 8, 11); }
                _ => { state.row = (state.row + board.height as i32 - 1) % board.height as i32; }
            }
        }
        Direction::Down => {
            match (state.row, state.col) {
                (7, 0..=3) => { state = state.rotate_about(TurnDirection::AROUND, 7, 3).translate(5, 4); }
                (7, 4..=7) => { state = state.rotate_about(TurnDirection::CCW, 7, 8); }
                (11, 8..=11) => { state = state.rotate_about(TurnDirection::AROUND, 11, 8).translate(-4, -5); }
                (11, 12..=15) => { state = state.rotate_about(TurnDirection::CCW, 11, 16).translate(-16, -8); }
                _ => { state.row = (state.row + 1) % board.height as i32; }
            }
        }
        Direction::Left => {
            match (state.row, state.col) {
                (0..=3, 8) => { state = state.rotate_about(TurnDirection::CCW, 4, 8); }
                (4..=7, 0) => { state = state.rotate_about(TurnDirection::CW, 7, 0).translate(4, 12); }
                (8..=11, 8) => { state = state.rotate_about(TurnDirection::CW, 7, 8); }
                _ => { state.col = (state.col + board.width  as i32 - 1) % board.width as i32; }
            }
        }
        Direction::Right => {
            match (state.row, state.col) {
                (0..=3, 11) => { state = state.rotate_about(TurnDirection::CW, 4, 11); }
                (4..=7, 11) => { state = state.rotate_about(TurnDirection::CW, 8, 11); }
                (8..=11, 15) => { state = state.rotate_about(TurnDirection::AROUND, 8, 15).translate(-5, -4); }
                _ => { state.col = (state.col + 1) % board.width as i32; }
            }
        }
    };
    state
}

fn move_as_cube(mut state: State, board: &Board) -> State {
    match state.direction {
        Direction::Up => {
            match (state.row, state.col) {
                (100, 0..=49) => { state = state.rotate_about(TurnDirection::CW, 100, 50); }  // A -- B
                (0, 50..=99) => { state = state.rotate_about(TurnDirection::CW, 0, 50).translate(150, -50); }  // D -- M (0, 50) -> (150, 0)
                (0, 100..=149) => { state = state.translate(199, -100); }  // E -- L (0, 100) -> (199, 0)
                _ => { state.row = (state.row + board.height as i32 - 1) % board.height as i32; }
            }
        }
        Direction::Down => {
            match (state.row, state.col) {
                (199, 0..=49) => { state = state.translate(-199, 100); }  // L -- E (199, 0) -> (0, 100)
                (149, 50..=99) => { state = state.rotate_about(TurnDirection::CW, 149, 49); }  // J -- K
                (49, 100..=149) => { state = state.rotate_about(TurnDirection::CW, 49, 99); }  // G -- H
                _ => { state.row = (state.row + 1) % board.height as i32; }
            }
        }
        Direction::Left => {
            match (state.row, state.col) {
                (0..=49, 50) => { state = state.rotate_about(TurnDirection::AROUND, 49, 50).translate(51, -50); }  // C -- N (49, 50) -> (100, 0)
                (50..=99, 50) => { state = state.rotate_about(TurnDirection::CCW, 100, 50); }  // B -- A
                (100..=149, 0) => { state = state.rotate_about(TurnDirection::AROUND, 100, 0).translate(-51, 50); }  // N -- C (100, 0) -> (49, 50)
                (150..=199, 0) => { state = state.rotate_about(TurnDirection::CCW, 150, 0).translate(-150, 50); }  // M -- D (150, 0) -> (0, 50)
                _ => { state.col = (state.col + board.width  as i32 - 1) % board.width as i32; }
            }
        }
        Direction::Right => {
            match (state.row, state.col) {
                (0..=49, 149) => { state = state.rotate_about(TurnDirection::AROUND, 49, 149).translate(51, -50); }  // F -- I (49, 149) -> (100, 99)
                (50..=99, 99) => { state = state.rotate_about(TurnDirection::CCW, 49, 99); }  // H -- G
                (100..=149, 99) => { state = state.rotate_about(TurnDirection::AROUND, 100, 99).translate(-51, 50); }  // I -- F (100, 99) -> (49, 149)
                (150..=199, 49) => { state = state.rotate_about(TurnDirection::CCW, 149, 49); }  // K -- J
                _ => { state.col = (state.col + 1) % board.width as i32; }
            }
        }
    };
    state
}

fn do_it_all(path: &str, processor: fn(State, &Board) -> State) -> i32 {
    let (board, actions) = load_data(path);
    let mut state = State {
        row: 0,
        col: board.data[0].iter().find_position(|c| *c != &' ').unwrap().0 as i32,
        direction: Direction::Right,
    };
    let mut path: Vec<(i32, i32, Direction)> = Vec::new();
    // println!("Start:");
    // board.print(&path);
    for action in &actions {
        //println!("Action: {:?}", action);
        match action {
            Action::Move(n) => {
                state = iterate(state, |&s| processor(s, &board))
                .filter(|&s| board.data[s.row as usize][s.col as usize] != ' ')
                .take(n+1)
                .take_while(|&s| board.data[s.row as usize][s.col as usize] == '.')
                .map(|s| {
                    path.push((s.row, s.col, s.direction));
                    s
                })
                .last()
                .unwrap();
            }
            Action::Turn(turn_direction) => {
                state.direction = turn(state.direction, *turn_direction);
            }
        }
        // board.print(&path);
        // println!("");
    }

    // println!("End:");
    // board.print(&path);
    let password = 1000 * (state.row + 1) + 4 * (state.col + 1) + match state.direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3
    };
    password
}

#[named]
fn part1_test() {
    let password = do_it_all("src\\d22\\data_test.txt", move_one_with_2d_wrap);
    println!("{}: {}", function_name!(), password);
}

#[named]
fn part1() {
    let password = do_it_all("src\\d22\\data.txt", move_one_with_2d_wrap);
    println!("{}: {}", function_name!(), password);
}

#[named]
fn part2_test() {
    let password = do_it_all("src\\d22\\data_test.txt", move_as_cube_test);
    println!("{}: {}", function_name!(), password);
}
#[named]

fn part2() {
    let password = do_it_all("src\\d22\\data.txt", move_as_cube);
    println!("{}: {}", function_name!(), password);
}

pub fn run() {
    part1_test();
    part2_test();
    part1();
    part2();
}

// Day22:
// part1_test: 6032
// part2_test: 5031
// part1: 149138
// part2: 153203