use ::function_name::named;
use core::panic;
use itertools::Itertools;
use std::fs;
use utilities::{Board, PointRC};

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
    Right,
}

fn get_step(direction: Direction) -> PointRC {
    match direction {
        Direction::Up => PointRC { r: -1, c: 0 },
        Direction::Down => PointRC { r: 1, c: 0 },
        Direction::Left => PointRC { r: 0, c: -1 },
        Direction::Right => PointRC { r: 0, c: 1 },
    }
}

fn load_data(path: &str, board_modifier_fn: fn(&str) -> String) -> (Board<char>, Vec<Direction>) {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let (board_str, moves_string) = file_contents_as_string
        .split("\r\n\r\n")
        .collect_tuple()
        .unwrap();
    let board_string = board_modifier_fn(board_str);
    let board = Board::<char>::load_data_chars_from_string(board_string.as_str());
    let moves = moves_string
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect();
    (board, moves)
}

#[named]
fn part1() {
    use real_data as data;

    let (mut board, moves) = load_data(data::FILENAME, |s| s.to_string());
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
    let res = board
        .data
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &value)| value == 'O')
                .map(|(c, _)| 100 * r + c)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{}: {}", function_name!(), res);
}

fn board_expander(board_in: &str) -> String {
    board_in
        .chars()
        .map(|c| match c {
            '#' => "##",
            'O' => "[]",
            '.' => "..",
            '@' => "@.",
            '\r' => "\r",
            '\n' => "\n",
            _ => panic!("Invalid character"),
        })
        .collect::<String>()
}

fn neighbor_step(c: char) -> PointRC {
    match c {
        '[' => PointRC { r: 0, c: 1 },
        ']' => PointRC { r: 0, c: -1 },
        _ => panic!("Invalid character"),
    }
}

fn attempt_move(
    board: &Board<char>,
    loc: PointRC,
    step: PointRC,
    second_half: bool,
    all_move_locations: &mut Vec<PointRC>,
) -> bool {
    //let step = get_step(dir);
    let horizontal = step.c != 0;
    let ret = match board[loc] {
        '.' => true,
        '#' => false,
        c @ '[' | c @ ']' => {
            let new_loc = loc.add(&step);
            let neighbor_loc = loc.add(&neighbor_step(c));
            assert!(board[neighbor_loc] == '[' || board[neighbor_loc] == ']');
            if (second_half || attempt_move(board, neighbor_loc, step, true, all_move_locations))
                && (horizontal || attempt_move(board, new_loc, step, false, all_move_locations))
            {
                if !all_move_locations.contains(&loc) {
                    all_move_locations.push(loc);
                }
                true
            } else {
                false
            }
        }
        _ => panic!("Invalid character"),
    };
    ret
}

#[named]
fn part2() {
    use test_data as data;

    let (mut board, moves) = load_data(data::FILENAME, board_expander);
    let mut robot_loc = board.find_first('@').unwrap();
    board[robot_loc] = '.';
    //robot_loc = PointRC { r: 4, c: 5 };
    for (i, direction) in moves.iter().enumerate() {
        //let direction = Direction::Left;
        let step = get_step(*direction);
        let new_robot_loc = robot_loc.add(&step);
        //board.print_with_actor(&robot_loc, '@');
        let mut all_move_locations = Vec::<PointRC>::new();
        let could_move = attempt_move(
            &board,
            new_robot_loc,
            step,
            false,
            &mut all_move_locations,
        );
        if could_move {
            all_move_locations.iter().for_each(|loc| {
                let mut new_loc = loc.add(&step);
                board[new_loc] = board[*loc];
                board[*loc] = '.';
            });
            robot_loc = new_robot_loc;
        }
        println!("{}: {:?}", i, direction);
        board.print_with_actor(&robot_loc, '@');
        println!("{}: {}:{:?}", function_name!(), could_move, all_move_locations);
    }
}

#[test]
fn test_part2() {
    part2();
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 1436690
// part2: 😱
