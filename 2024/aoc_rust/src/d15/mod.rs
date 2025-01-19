use itertools::Itertools;
use std::fs;
use utilities::{Board, PointRC};
use ::function_name::named;

const DAY: &str = "d15";

mod test_data {
    pub const FILENAME: &str = r"src\d15\data_test.txt";
}
mod real_data {
    pub const FILENAME: &str = r"src\d15\data.txt";
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn get_step(direction: Direction) -> PointRC {
    match direction {
        Direction::Up => PointRC { r: -1, c: 0 },
        Direction::Down => PointRC { r: 1, c: 0 },
        Direction::Left => PointRC { r: 0, c: -1 },
        Direction::Right => PointRC { r: 0, c: 1 }
    }
}

fn load_data(path: &str) -> (Board<char>, Vec<Direction>) { 
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let (board_string, moves_string) = file_contents_as_string.split("\r\n\r\n").collect_tuple().unwrap();
    let board = Board::<char>::load_data_chars_from_string(board_string);
    let moves = moves_string.chars().filter(|c| !c.is_whitespace()).map(|c| match c {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => panic!("Invalid direction")
    }).collect();
    (board, moves)
}

#[named]
fn part1() {
    use real_data as data;

    let (mut board, moves) = load_data(data::FILENAME);
    let mut robot_loc = board.find_first('@').unwrap();
    board[robot_loc] = '.';
    //board.print_with_actor(&robot_loc, '@');
    for (_, direction) in moves.iter().enumerate() {
        let step = get_step(*direction);
        let new_loc = robot_loc.add(&step);
        let mut push_into = new_loc;
        while board[push_into] == 'O' {
            push_into = push_into.add(&step);
        }
        assert!(board[push_into] == '.' || board[push_into] == '#');
        if board[push_into] == '.' {
            board[push_into] = 'O';
            board[new_loc] = '.';
            robot_loc = new_loc;
        }
        //println!("{}: {:?}", i, direction);
        //board.print_with_actor(&robot_loc, '@');
        // if i > 10 {
        //     break;
        // }
    }
    //board.print_with_actor(&robot_loc, '@');
    let res = board.data.iter().enumerate().map(|(r, row)| {
        row.iter().enumerate().filter(|(_, &value)| value == 'O').map(|(c, _)| 100 * r + c).sum::<usize>()
    }).sum::<usize>();

    println!("{}: {}", function_name!(), res);
}

#[named]
fn part2() {
    use test_data as data;

    let (board, moves) = load_data(data::FILENAME);
    board.print();
    println!("{}: {}", function_name!(), board.data[0][0]);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 1436690
// part2: 😱
