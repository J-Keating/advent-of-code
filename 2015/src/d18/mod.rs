use ::function_name::named;
use utilities::alloc_2d_vec;
use std::{fs};

const DAY: i32 = 18;

#[derive(Copy, Clone, PartialEq)]
enum State {
    On,
    Off,
}

fn load_data(filename: &str) -> Vec<Vec<State>> {
    let file_contents_as_string = fs::read_to_string(filename).unwrap();
    let file_lines = file_contents_as_string.lines().collect::<Vec<&str>>();
    let height = file_lines.len();
    let width = file_lines[0].len();
    let mut grid = alloc_2d_vec(height+2, width+2, State::Off);
    for (row, line) in file_lines.iter().enumerate() {
        for (col, state) in line.chars().enumerate() {
            grid[row+1][col+1] = match state {
                '#' => State::On,
                '.' => State::Off,
                _ => panic!("Unknown state"),
            };
        }
    }
    grid
}

#[allow(dead_code)]
fn print_lights(grid: &Vec<Vec<State>>) {
    for row in grid.iter() {
        println!("{}", row.iter().fold(String::new(), |acc, &v| acc + match v {
            State::On => "#",
            State::Off => ".",
        }));
    }
}

fn animate_lights(curr_state: &Vec<Vec<State>>) -> Vec<Vec<State>> {
    let mut next_state = alloc_2d_vec(curr_state.len(), curr_state[0].len(), State::Off);
    for row in 1..curr_state.len()-1 {
        for col in 1..curr_state[0].len()-1 {
            let mut num_on = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 { continue; }
                    if curr_state[(row as i32 + i) as usize][(col as i32 + j) as usize] == State::On {
                        num_on += 1;
                    }
                }
            }
            next_state[row][col] = match curr_state[row][col] {
                State::On => if num_on == 2 || num_on == 3 { State::On } else { State::Off },
                State::Off => if num_on == 3 { State::On } else { State::Off },
            };
        }
    }
    next_state
}

fn count_on(grid: &Vec<Vec<State>>) -> i32 {
    grid.iter().fold(0, |acc, row| acc + row.iter().fold(0, |acc, &v| acc + match v {
        State::On => 1,
        State::Off => 0,
    }))
}

#[named]
fn part1_test() {
    let mut grid = load_data(&format!("src\\d{}\\data_test.txt", DAY));
    //print_lights(&grid);
    for _ in 0..4 {
        grid = animate_lights(&grid);
        //print_lights(&grid);
    }
    println!("{}: {}", function_name!(), count_on(&grid));
}

#[named]
fn part1() {
    let mut grid = load_data(&format!("src\\d{}\\data.txt", DAY));
    for _ in 0..100 {
        grid = animate_lights(&grid);
    }
    println!("{}: {}", function_name!(), count_on(&grid));
}

#[named]
fn part2_test() {
    let mut grid = load_data(&format!("src\\d{}\\data_test.txt", DAY));
    let height = grid.len();
    let width = grid[0].len();
    grid[1][1] = State::On;
    grid[1][width-2] = State::On;
    grid[height-2][1] = State::On;
    grid[height-2][width-2] = State::On;
    //print_lights(&grid);
    for _ in 0..5 {
        grid = animate_lights(&grid);
        grid[1][1] = State::On;
        grid[1][width-2] = State::On;
        grid[height-2][1] = State::On;
        grid[height-2][width-2] = State::On;
        //print_lights(&grid);
    }
    println!("{}: {}", function_name!(), count_on(&grid));
}

#[named]
fn part2() {
    let mut grid = load_data(&format!("src\\d{}\\data.txt", DAY));
    let height = grid.len();
    let width = grid[0].len();
    grid[1][1] = State::On;
    grid[1][width-2] = State::On;
    grid[height-2][1] = State::On;
    grid[height-2][width-2] = State::On;
    for _ in 0..100 {
        grid = animate_lights(&grid);
        grid[1][1] = State::On;
        grid[1][width-2] = State::On;
        grid[height-2][1] = State::On;
        grid[height-2][width-2] = State::On;
    }
    println!("{}: {}", function_name!(), count_on(&grid));
}

pub fn run() {
    println!("Day {}:", DAY);
    part1_test();
    part1();
    part2_test();
    part2();
}

// part1_test: 4
// part1: 821
// part2_test: 17
// part2: 886
